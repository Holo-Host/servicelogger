
use std::convert::From;

use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_persistence_api::{
        hash::HashString,
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
pub struct ClientRequest {
    agent_id: Agent, // ed25519 public key, in "HcSc..." form
    instance_id: HashString, // SHA256 hash of the Conductor DNA Instance being targeted, in "Qm..." form
    call_spec: String, // For categorization, eg. blog/create_post
    request_digest: Digest, // SHA256 Digest of full zome call spec + args, in "Qm..." form
    agent_signature: AgentSignature, // ed25519 signature of request_hash, in base-64 form
}

pub fn client_request_definition() -> ValidatingEntryType {
    entry!(
        name: "client_request",
        description: "this it the entry defintion for a client request",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<ClientRequest>| {
            validate_request(_validation_data)
        }
    )
}

fn validate_request(context: EntryValidationData<ClientRequest>) -> Result<(), String> {
    match context {
        EntryValidationData::Create{entry:client_request, validation_data: _} => {
            // The Client Agent must have signed the `client_request.request_digest`.

            // Base-58 -> Result<Vec<u8>, FromBase58Error>
            if client_request.agent_id.verify(
                client_request.request_digest.to_bytes(),
                &client_request.agent_signature
            ) {
                Ok(())
            } else {
                Err(format!(
                    "Signature invalid for ClientRequest {} with Agent {} and Signature {}",
                    client_request.request_digest,
                    client_request.agent_id,
                    client_request.agent_signature
                ))
            }
        } 
        _ => {
            Err("Failed to validate with wrong entry type".to_string())
        }
    }
}


pub fn handle_log_request(entry: ClientRequest) -> ZomeApiResult<Address> {
    let entry = Entry::App("client_request".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_request(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
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
  "instance_id": "QmfAzihC8RVNLCwtDeeUH8eSAACweFq77KBK4e1bJWmU8A",
  "call_spec": "blog/create_post",
  "request_digest": "{}",
  "agent_signature": "{}"
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
        assert_eq!(client_request.agent_signature.clone().to_bytes()[..32], sig_bytes);
        assert_eq!(
            AgentSignature::from_bytes(&client_request.agent_signature.clone().to_bytes()).unwrap(),
            client_request.agent_signature
        );
    }
}
