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
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry, error::HolochainError, json::JsonString,
};

mod request;
mod response;
mod servicelog;
mod setup;

// see https://developer.holochain.org/api/0.0.3/hdk/ for info on using the hdk library

define_zome! {
    entries: [
        setup::payment_prefs_definition(),
        request::client_request_definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        set_payment_prefs: {
            inputs: |entry: setup::PaymentPrefs|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: setup::handle_set_payment_prefs
        }
        log_request: {
            inputs: |entry: request::ClientRequest|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: request::handle_log_request
        }
        get_request: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<Option<Entry>>|,
            handler: request::handle_get_request
        }
    ]

    capabilities: {
        public (Public) [set_payment_prefs, log_request, get_request]
    }
}
