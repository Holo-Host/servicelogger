
use std::convert::From;

use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::{
            Address,
        },
    },
    holochain_json_api::{
        json::JsonString, error::JsonError,
    },
    holochain_core_types::{
        entry::Entry,
        dna::entry_types::Sharing,
        validation::EntryValidationData,

    },
};

use crate::validate::*; // Agent, AgentSignature, Digest, ...

/// ClientRequest represents the start of a unique Client Agent interaction with a Holochain
/// Instance in this Host.  Its commit's ChainHeader contains the Hosts' timestamp indicating when
/// it started to service the request.
///
/// TODO: Necessary? Each ServiceLogger instance is associated with an conductor DNA Instance...
/// 
/// TODO: Is call_spec necessary? Until the corresponding HostResponse is committed, we can not
/// deduce resource utilization (other than elapsed wall-clock time of request thus far). This may
/// be useful, to see what kind(s) of requests are blocking for long durations.
/// 
/// TODO: An Address = HashString(String) does no validation on deserialization.  If it is converted
/// to/from bytes, it is assumed to be a base-58 encoded multihash hash::SHA256 (ie. "Qm..."); see
/// holochain-persistence/crates/holochain_persistence_api/src/hash.rs.  We should use an Agent
/// type, which de/serializes to hcid::HcidEncoding::with_kind("hcs0"): "HcScJ...".  Likewise,
/// Signature does no validation.

#[derive(Debug, Clone, DefaultJson, Serialize, Deserialize)]
pub struct CallSpec {
    pub zome: String,
    pub function: String,
}

#[derive(Debug, Clone, DefaultJson, Serialize, Deserialize)]
pub struct ClientPayload {
    pub hash: Digest,
    pub signature: AgentSignature,
}

#[derive(Debug, Clone, DefaultJson, Serialize, Deserialize)]
pub struct ClientRequest {
    pub agent_id: Agent,
    pub call_spec: CallSpec,
    pub payload: ClientPayload,
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

fn validate_request(
    context: EntryValidationData<ClientRequest>
) -> Result<(), String> {
    match context {
        EntryValidationData::Create{entry:client_request, validation_data: _} => {
            // The Client Agent must have signed the `client_request.request_digest`.

            // Base-58 -> Result<Vec<u8>, FromBase58Error>
            if client_request.agent_id.verify(
                client_request.payload.hash.to_bytes(),
                &client_request.payload.signature
            ) {
                Ok(())
            } else {
                Err(format!(
                    "Signature invalid for Agent {}, for ClientRequest Digest {} and Signature {}",
                    client_request.agent_id,
                    client_request.payload.hash,
                    client_request.payload.signature
                ))
            }
        } 
        _ => {
            Err("Failed to validate with wrong entry type".to_string())
        }
    }
}


pub fn handle_log_request(
    agent_id: Agent,
    call_spec: CallSpec,
    payload: ClientPayload
) -> ZomeApiResult<Address> {
    let entry = Entry::App(
        "client_request".into(),
        ClientRequest { agent_id, call_spec, payload }.into()
    );
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

#[derive(Debug, Clone, DefaultJson, Serialize, Deserialize)]
pub struct ClientRequestMeta {
    pub meta: CommitMeta,
    pub request: ClientRequest,
}

pub fn handle_get_request(
    address: Address
) -> ZomeApiResult<ClientRequestMeta> {
    let (meta,request) = get_meta_and_entry_as_type::<ClientRequest>(address)?;
    Ok(ClientRequestMeta { meta, request })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek;
    use std::{str::FromStr, convert::{From, TryFrom, }, };

    #[test]
    fn client_request_smoke() {

        // Get a legit request_hash signature, agent_id
        let secret: [u8; 32] = [0_u8; 32];
        let secret_key = ed25519_dalek::SecretKey::from_bytes( &secret ).unwrap();
        let public_key = ed25519_dalek::PublicKey::from( &secret_key );
        let secret_key_exp = ed25519_dalek::ExpandedSecretKey::from( &secret_key );
        let agent_id =  Agent::from( &secret_key );
        assert_eq!( Agent::from( public_key.clone() ), agent_id );
        assert_eq!( Agent::try_from( public_key.to_bytes().to_vec().as_slice() ).unwrap(), agent_id );
        
        
        // Lets make up a request digest, and test Digest round-tripping
        let request_digest_bytes: [u8; 32] = [0_u8; 32];
        let request_digest = Digest::from( request_digest_bytes.clone() );
        assert_eq!( request_digest.to_bytes(), &request_digest_bytes );
        let request_digest = Digest::from_bytes( &request_digest_bytes ).unwrap();
        assert_eq!( request_digest.to_bytes(), &request_digest_bytes );

        let request_digest_str = "QmNLei78zWmzUdbeRB3CiUfAizWUrbeeZh5K1rhAQKCh51";

        let request_digest_def = format!("{}", &request_digest);
        assert_eq!( request_digest_def, request_digest_str );

        assert_eq!( Digest::from_str(request_digest_str).unwrap().to_bytes(),
                    &request_digest_bytes );
        
        let request_digest_json = format!(r#""{}""#, request_digest_str);
        let request_digest_pty = serde_json::to_string_pretty( &request_digest ).unwrap();
        assert_eq!( request_digest_pty, request_digest_json );
        let request_digest_json_rec: Digest = serde_json::from_str(&request_digest_json).unwrap();
        assert_eq!( request_digest_json_rec, request_digest );
        assert_eq!( request_digest_json_rec.to_bytes(), &request_digest_bytes );

        let signature = secret_key_exp.sign( request_digest.to_bytes(), &public_key );
        let agent_signature = AgentSignature::from_bytes( &signature.to_bytes() ).unwrap();

        let client_request_str = format!(
            r#"{{
  "agent_id": "{}",
  "call_spec": {{
    "zome": "blog",
    "function": "create_post"
  }},
  "payload": {{
    "hash": "{}",
    "signature": "{}"
  }}
}}"#,
            agent_id, request_digest, agent_signature );
        println!("ClientRequest: {}", client_request_str);
        let client_request: ClientRequest = serde_json::from_str(&client_request_str).unwrap();

        let digest2 = Digest::from([1_u8; 32]);
        let signature2 = secret_key_exp.sign( digest2.to_bytes(), &public_key );
        let agent_signature2 = AgentSignature::from_bytes( &signature2.to_bytes() ).unwrap();
        println!("Digest: {}, AgentSignature: {}", digest2, agent_signature2);
        
        let digest3 = Digest::from([0xff_u8; 32]);
        let signature3 = secret_key_exp.sign( digest3.to_bytes(), &public_key );
        let agent_signature3 = AgentSignature::from_bytes( &signature3.to_bytes() ).unwrap();
        println!("Digest: {}, AgentSignature: {}", digest3, agent_signature3);
        
        
        assert_eq!( serde_json::to_string_pretty( &client_request ).unwrap(), client_request_str);

        // Our agent_id should be deterministic
        let id_bytes: [u8; 32] = [
            59, 106, 39, 188, 206, 182, 164, 45,
            98, 163, 168, 208, 42, 111, 13, 115,
            101, 50, 21, 119, 29, 226, 67, 166,
            58, 192, 72, 161, 139, 89, 218, 41
        ];
        assert_eq!(client_request.agent_id.to_bytes(), id_bytes);
        // Just 1st 32 bytes of 64-byte sig, for ease of testing
        let sig_bytes: [u8; 32] = [
            61, 161, 235, 223, 169, 110, 221, 24,
            29, 190, 54, 89, 209, 192, 81, 196,
            49, 240, 86, 165, 173, 106, 151, 166,
            13, 92, 202, 16, 70, 4, 56, 120
        ];
        // Round-trip the AgentSignature
        assert_eq!(client_request.payload.signature.clone().to_bytes()[..32], sig_bytes);
        assert_eq!(
            AgentSignature::from_bytes(&client_request.payload.signature.clone().to_bytes()).unwrap(),
            client_request.payload.signature
        );
    }
}
