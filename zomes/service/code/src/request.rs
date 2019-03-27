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
pub struct ClientRequest {
    agent_id: Address,
    zome_call_spec: String,
    dna_hash: HashString,
    client_signature: HashString, // signature contains dna_hash + zome_call_spec 
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
        EntryValidationData::Create{entry:_obj,validation_data:_} => {
            Ok(())
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
