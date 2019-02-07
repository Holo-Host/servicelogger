#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry, dna::entry_types::Sharing, error::HolochainError, json::JsonString,
};

// see https://developer.holochain.org/api/0.0.3/hdk/ for info on using the hdk library

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct ClientRequest {
    content: String,
}

pub fn handle_create_request(entry: ClientRequest) -> ZomeApiResult<Address> {
    let entry = Entry::App("client_request".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_request(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

fn definition() -> ValidatingEntryType {
    entry!(
        name: "client_request",
        description: "this is a same entry defintion",
        sharing: Sharing::Public,
        native_type: ClientRequest,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_my_entry: ClientRequest, _validation_data: hdk::ValidationData| {
            Ok(())
        }
    )
}
define_zome! {
    entries: [
       definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        create_request: {
            inputs: |entry: ClientRequest|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_request
        }
        get_request: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<Option<Entry>>|,
            handler: handle_get_request
        }
    ]

    capabilities: {
        public (Public) [create_request,get_request]
    }
}
