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
        validation::EntryAction
    },
};
// use serde::Serialize;
// use serde_json::{self, Value};

use super::setup;

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct HostingStats {
    pub cpu_seconds: f64,
    pub bytes_in: usize,
    pub bytes_out: usize,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct HostResponse {
    request_hash: HashString,
    hosting_stats: HostingStats,
    response_log: String,
    host_signature: HashString,
}

pub fn host_response_definition() -> ValidatingEntryType {
    entry!(
        name: "host_response",
        description: "this it the entry defintion for a host response",
        sharing: Sharing::Public,
        native_type: HostResponse,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |entry: HostResponse, context: hdk::ValidationData| {
            validate_response(entry, context)
        }
    )
}

fn validate_response(entry: HostResponse, context: hdk::ValidationData) -> Result <(), String> {
    if setup::get_latest_payment_prefs().is_none() {
        return Err("Payment prefs not set, please perform setup prior to creating other entries".to_string())
    }

    match context.action {
        EntryAction::Create => match entry {
            // TODO: validate if signature is valid
            HostResponse { request_hash: hash, .. } => match hdk::get_entry(&hash) {
                Ok(maybe_entry) => match maybe_entry {
                    Some(_) => Ok(()),
                    None => Err("ClientRequest entry not found!".to_string())
                }
                Err(e) => Err(e.to_string())
            },
        },
        _ => Err(format!("Invalid action for {:?}", entry)),
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
