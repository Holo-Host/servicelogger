#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate holochain_wasm_utils;
extern crate holochain_persistence_api;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::{
            Address,
        },
    },
    holochain_json_api::{
        json::JsonString, error::JsonError,
    },
    holochain_core_types::{
        entry::Entry,
    },
};

mod request;
mod response;
mod servicelog;
mod invoice;
mod setup;

// see https://developer.holochain.org/api/0.0.4/hdk/ for info on using the hdk library

define_zome! {
    entries: [
        setup::setup_prefs_definition(),
        request::client_request_definition(),
        response::host_response_definition(),
        servicelog::service_log_definition(),
        invoice::invoiced_logs_definition()
    ]

    init: || { Ok(()) }

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

    functions: [
        setup: {
            inputs: |entry: setup::SetupPrefs|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: setup::handle_setup
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
        list_uninvoiced_servicelogs: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: servicelog::handle_list_uninvoiced_servicelogs
        }
        generate_invoice: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Option<Address>>|,
            handler: invoice::handle_generate_invoice
        }
        list_unpaid_invoices: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: invoice::handle_list_unpaid_invoices
        }
        get_payment_status: {
            inputs: | |,
            outputs: |result: ZomeApiResult<invoice::PaymentStatus>|,
            handler: invoice::handle_get_payment_status
        }
    ]

    traits: {
        hc_public [
            setup,
            log_request,
            get_request,
            log_response,
            get_response,
            log_service,
            get_service,
            list_uninvoiced_servicelogs,
            generate_invoice,
            list_unpaid_invoices,
            get_payment_status
        ]
    }
}
