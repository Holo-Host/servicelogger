#[allow(unused_imports)]
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        entry::Entry,
        error::HolochainError,
        hash::HashString,
        json::{DefaultJson, JsonString},
        dna::entry_types::Sharing,
        cas::content::Address,
        validation::EntryAction,
        validation::EntryValidationData
    },
};
// use serde::Serialize;
// use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct HostingStats {
    pub cpu_seconds: f64,
    pub bytes_in: usize,
    pub bytes_out: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct HostResponse {
    pub request_hash: HashString,
    pub hosting_stats: HostingStats,
    pub response_data_hash: HashString,
    pub response_log: String, // not included into the signature
    pub host_signature: HashString, // signed request_hash + response_data_hash + hosting_stats
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
            HostResponse { request_hash: hash, .. } => match hdk::get_entry(&hash) {
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
