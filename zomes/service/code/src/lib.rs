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
        request::client_request_definition(),
        response::host_response_definition(),
        servicelog::service_log_definition()
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
        log_response: {
            inputs: |entry: response::HostResponse|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: response::handle_log_response
        }
        get_response: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<Option<Entry>>|,
            handler: response::handle_get_response
        }
        log_service: {
            inputs: |entry: servicelog::ServiceLog|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: servicelog::handle_log_service
        }
        get_service: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<Option<Entry>>|,
            handler: servicelog::handle_get_service
        }
        list_servicelogs: {
            inputs: | |,
            outputs: |result: Vec<Entry>|,
            handler: servicelog::handle_list_servicelogs
        }
    ]

    traits: {
        hc_public [
            set_payment_prefs,
            log_request,
            get_request,
            log_response,
            get_response,
            log_service,
            get_service,
            list_servicelogs
        ]
    }
}
