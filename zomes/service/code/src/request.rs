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
            // TODO: validate if payment_prefs is set
            Ok(())
        }
    )
}

pub fn handle_log_request(entry: ClientRequest) -> ZomeApiResult<Address> {
    let entry = Entry::App("client_request".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_request(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}
