use ed25519_dalek;
use std::convert::From; // {Signature, PublicKey}

use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{
        dna::entry_types::Sharing, entry::Entry, time::Iso8601, validation::EntryValidationData,
    },
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};

use crate::validate::AGENT_CODEC;
use crate::validate::*; // Agent, AgentSignature, Digest, ...

/// ClientRequest represents the start of a unique Client Agent interaction with a Holochain
/// Instance in this Host.  Its commit's ChainHeader contains the Hosts' timestamp indicating when
/// it started to service the request.
///
/// TODO: Multiple servicelogger's Necessary? Each ServiceLogger instance is associated with an
/// conductor hApp hash and its DNA Instance(s). The ClientRequest record *uniquely* identifies the
/// hApp, so a single servicelogger instance can track all request to all hApps.
///
#[derive(Debug, Clone, DefaultJson, Serialize, Deserialize)]
pub struct ClientRequest {
    pub agent_id: Address,
    pub request: RequestPayload,
    pub request_signature: AgentSignature,
}

#[derive(Debug, Clone, DefaultJson, Serialize, Deserialize)]
pub struct RequestPayload {
    call_spec: CallSpec,
    host_id: Address,
    timestamp: Iso8601,
}

#[derive(Debug, Clone, DefaultJson, Serialize, Deserialize)]
pub struct CallSpec {
    pub args_hash: Digest,
    pub dna_alias: String,
    pub function: String,
    pub hha_hash: Digest,
    pub zome: String,
}

pub fn client_request_definition() -> ValidatingEntryType {
    entry!(
        name: "client_request",
        description: "Start of a client Agent request",
        sharing: Sharing::Private,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |validation_data: hdk::EntryValidationData<ClientRequest>| {
            validate_request(validation_data)
        }
    )
}

/// Validate the Private ClientRequest entry.
fn validate_request(context: EntryValidationData<ClientRequest>) -> Result<(), String> {
    match context {
        EntryValidationData::Create {
            entry: client_request,
            validation_data: _,
        } => {
            // The Client Agent must have signed a standard serialization of the request
            let request_serialization =
                serde_json::to_string(&client_request.request).map_err(|e| e.to_string())?;
            let agent_id = AGENT_CODEC
                .decode(&client_request.agent_id.to_string())
                .unwrap();
            let agent_key: Agent = ed25519_dalek::PublicKey::from_bytes(&agent_id)
                .unwrap()
                .into();
            // let agent_key: Agent = agent_key;
            if !agent_key.verify(
                &request_serialization.as_bytes(),
                &client_request.request_signature,
            ) {
                return Err(format!(
                    "Signature {} invalid for request payload: {}",
                    &client_request.request_signature, &request_serialization
                ));
            };
        }
        _ => return Err(String::from("Failed to validate with wrong entry type")),
    }
    Ok(())
}

pub fn handle_log_request(
    agent_id: Address,
    request: RequestPayload,
    request_signature: AgentSignature,
) -> ZomeApiResult<Address> {
    let entry = Entry::App(
        "client_request".into(),
        ClientRequest {
            agent_id,
            request,
            request_signature,
        }
        .into(),
    );
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

#[derive(Debug, Clone, DefaultJson, Serialize, Deserialize)]
pub struct ClientRequestMeta {
    pub meta: CommitMeta,
    pub client_request: ClientRequest,
}

pub fn handle_get_request(address: Address) -> ZomeApiResult<ClientRequestMeta> {
    let (meta, client_request) = get_meta_and_entry_as_type::<ClientRequest>(address)?;
    Ok(ClientRequestMeta {
        meta,
        client_request,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek;
    use std::{
        convert::{From, TryFrom},
        str::FromStr,
    };

    #[test]
    fn client_request_smoke() {
        // Get a legit request_hash signature, agent_id
        let secret: [u8; 32] = [0_u8; 32];
        let secret_key = ed25519_dalek::SecretKey::from_bytes(&secret).unwrap();
        let public_key = ed25519_dalek::PublicKey::from(&secret_key);
        let secret_key_exp = ed25519_dalek::ExpandedSecretKey::from(&secret_key);
        let agent_id = Agent::from(&secret_key);
        assert_eq!(Agent::from(public_key.clone()), agent_id);
        assert_eq!(
            Agent::try_from(public_key.to_bytes().to_vec().as_slice()).unwrap(),
            agent_id
        );

        // Lets make up a request args_hash, and test Digest round-tripping
        let args_hash_bytes = [0_u8; 32];
        let args_hash = Digest::from(args_hash_bytes.clone());
        assert_eq!(args_hash.to_bytes(), &args_hash_bytes);
        let args_hash = Digest::from_bytes(&args_hash_bytes).unwrap();
        assert_eq!(args_hash.to_bytes(), &args_hash_bytes);

        let args_hash_str = "QmNLei78zWmzUdbeRB3CiUfAizWUrbeeZh5K1rhAQKCh51";

        let args_hash_def = format!("{}", &args_hash);
        assert_eq!(args_hash_def, args_hash_str);

        assert_eq!(
            Digest::from_str(args_hash_str).unwrap().to_bytes(),
            &args_hash_bytes
        );

        let args_hash_json = format!(r#""{}""#, args_hash_str);
        let args_hash_pty = serde_json::to_string_pretty(&args_hash).unwrap();
        assert_eq!(args_hash_pty, args_hash_json);
        let args_hash_json_rec: Digest = serde_json::from_str(&args_hash_json).unwrap();
        assert_eq!(args_hash_json_rec, args_hash);
        assert_eq!(args_hash_json_rec.to_bytes(), &args_hash_bytes);

        // We'll test with the computed Client ID, and a Host Agent ID that happens to match our
        // scenario tests.
        //
        // TODO: when hdk::sign allows signing of arbitrary data (ie. doesn't fail on JSON-encoded
        // data w/ escapes), we can arrange to sign the request from the perspective of any
        // "testing" Hosting Agnt ID.  Until then -- we can only successfully test in the hands of a
        // "testing" Hosting Agent ID matching this:
        let host_id = "HcScJhCTAB58mkeen7oKZrgxga457b69h7fV8A9CTXdTvjdx74fTp33tpcjitgz";
        let client_request_str = format!(
            r#"{{
  "agent_id": "{}",
  "request": {{
    "call_spec": {{
      "args_hash": "{}",
      "dna_alias": "openbook",
      "function": "create_post",
      "hha_hash": "QmNLei78zWmzUdbeRB3CiUfAizWUrbeeZh5K1rhAQKCh51",
      "zome": "blog"
    }},
    "host_id": "{}",
    "timestamp": "2019-11-25T05:48:34.123+07:00"
  }},
  "request_signature": "XxHr36xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxCg=="
}}"#,
            agent_id, args_hash, host_id
        );

        // Check round-tripping (not a valid request_signature yet, though...)
        //println!("ClientRequest str: {}", &client_request_str);
        let mut client_request: ClientRequest = serde_json::from_str(&client_request_str).unwrap();

        assert_eq!(
            serde_json::to_string_pretty(&client_request).unwrap(),
            client_request_str
        );

        let signature = secret_key_exp.sign(
            serde_json::to_string(&client_request.request)
                .unwrap()
                .as_bytes(),
            &public_key,
        );
        client_request.request_signature =
            AgentSignature::from_bytes(&signature.to_bytes()).unwrap();

        println!(
            "ClientRequest 1 valid: {}",
            serde_json::to_string_pretty(&client_request).unwrap()
        );

        // Lets get another w/ different args_hash value
        client_request.request.call_spec.args_hash = Digest::from([0xff_u8; 32]);
        let signature2 = secret_key_exp.sign(
            serde_json::to_string(&client_request.request)
                .unwrap()
                .as_bytes(),
            &public_key,
        );
        client_request.request_signature =
            AgentSignature::from_bytes(&signature2.to_bytes()).unwrap();
        println!(
            "ClientRequest 2 valid: {}",
            serde_json::to_string_pretty(&client_request).unwrap()
        );
    }
}
