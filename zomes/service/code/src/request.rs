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
pub struct ClientRequest {
    agent_id: Address,
    zome_call_spec: String,
    dna_hash: HashString,
    client_signature: HashString,
}

pub fn client_request_definition() -> ValidatingEntryType {
    entry!(
        name: "client_request",
        description: "this it the entry defintion for a client request",
        sharing: Sharing::Public,
        native_type: ClientRequest,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_my_entry: ClientRequest, _validation_data: hdk::ValidationData| {
            validate_request(_my_entry, _validation_data)
        }
    )
}

fn validate_request(_entry: ClientRequest, _context: hdk::ValidationData) -> Result <(), String> {
    if setup::get_latest_payment_prefs().is_none() {
        return Err("Payment prefs not set, please perform setup prior to creating other entries".to_string())
    }

    Ok(())
}

pub fn handle_log_request(entry: ClientRequest) -> ZomeApiResult<Address> {
    let entry = Entry::App("client_request".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_request(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}
