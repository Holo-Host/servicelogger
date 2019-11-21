
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
        validation::EntryValidationData
    },
};

use crate::validate::*; // AgentId, AgentSignature, Digest, ...

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct HostingStats {
    pub cpu_seconds: f64,
    pub bytes_in: usize,
    pub bytes_out: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct HostResponse {
    pub request_commit: Address,
    pub response_digest: Digest,
    pub hosting_stats: HostingStats,
    pub response_log: String,
    pub host_signature: AgentSignature
}

pub fn host_response_definition() -> ValidatingEntryType {
    entry!(
        name: "host_response",
        description: "this it the entry defintion for a host response",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |context: hdk::EntryValidationData<HostResponse>| {
            validate_response(context)
        }
    )
}

fn validate_response(context: EntryValidationData<HostResponse>) -> Result<(), String> {
    match context {
        EntryValidationData::Create{entry:obj,validation_data:_} => match obj {
            // Ensures that the response references an existing 
            HostResponse { request_commit: hash, .. } => match hdk::get_entry(&hash) {
                Ok(maybe_entry) => match maybe_entry {
                    Some(_) => Ok(()),
                    None => Err("ClientRequest entry not found!".to_string())
                }
                Err(e) => Err(e.to_string())
            },
        }
        _ => {
            Err("Failed to validate with wrong entry type".to_string())
        }
    }
}

pub fn handle_log_response(entry: HostResponse) -> ZomeApiResult<Address> {
    let entry = Entry::App("host_response".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_response(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}
