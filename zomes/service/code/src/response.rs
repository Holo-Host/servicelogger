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
        cas::content::Address
    },
};
// use serde::Serialize;
// use serde_json::{self, Value};

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

        validation: |_my_entry: HostResponse, _validation_data: hdk::ValidationData| {
            Ok(())
        }
    )
}

// TODO: disallow logging requests if payment prefs not set
pub fn handle_log_response(entry: HostResponse) -> ZomeApiResult<Address> {
    let entry = Entry::App("host_response".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_response(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}
