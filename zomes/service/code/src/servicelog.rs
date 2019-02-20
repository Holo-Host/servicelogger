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
pub struct ServiceLog {
    response_hash: HashString,
    client_signature: HashString,
}

pub fn service_log_definition() -> ValidatingEntryType {
    entry!(
        name: "service_log",
        description: "this it the entry defintion for a service log",
        sharing: Sharing::Public,
        native_type: ServiceLog,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_my_entry: ServiceLog, _validation_data: hdk::ValidationData| {
            Ok(())
        }
    )
}

// TODO: disallow logging requests if payment prefs not set
pub fn handle_log_service(entry: ServiceLog) -> ZomeApiResult<Address> {
    let entry = Entry::App("service_log".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_service(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}
