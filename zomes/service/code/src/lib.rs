#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate holochain_persistence_api;
extern crate holochain_wasm_utils;
#[macro_use]
extern crate holochain_json_derive;

#[macro_use]
extern crate failure;

extern crate hdk_proc_macros;
use hdk::prelude::*;
use hdk_proc_macros::zome;

use hdk::{AGENT_ADDRESS, AGENT_ID_STR, DNA_ADDRESS, DNA_NAME};

use std::convert::TryInto;

mod invoice;
mod request;
mod response;
mod servicelog;
mod setup;
mod validate;

#[zome]
pub mod service {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn setup_prefs_definition() -> ValidatingEntryType {
        setup::setup_prefs_definition()
    }
    #[entry_def]
    fn client_request_definition() -> ValidatingEntryType {
        request::client_request_definition()
    }
    #[entry_def]
    fn host_response_definition() -> ValidatingEntryType {
        response::host_response_definition()
    }
    #[entry_def]
    fn service_log_definition() -> ValidatingEntryType {
        servicelog::service_log_definition()
    }
    #[entry_def]
    fn invoiced_logs_definition() -> ValidatingEntryType {
        invoice::invoiced_logs_definition()
    }

    #[zome_fn("hc_public")]
    fn whoami() -> ZomeApiResult<WhoamiResult> {
        whoami_handler()
    }

    #[zome_fn("hc_public")]
    fn setup(entry: setup::SetupPrefs) -> ZomeApiResult<Address> {
        setup::handle_setup(entry)
    }

    /// For testing, we sometimes need to sign arbitrary things, as the Hosting agent
    #[zome_fn("hc_public")]
    fn sign(payload: String) -> ZomeApiResult<String> {
        hdk::sign(payload)
    }

    /// Start of a request by `agent_id`.  The `request_signature` is the `agent_id`'s signature of
    /// the serialization of [host_id, timestamp, [hha_hash, dna_alias, zome, function, args_hash]].
    /// The `host_id` *must* be that of the committing host for validation to succeed.
    #[zome_fn("hc_public")]
    fn log_request(
        agent_id: Address,
        request: request::RequestPayload,
        request_signature: validate::AgentSignature,
    ) -> ZomeApiResult<Address> {
        Ok(Address::from("Mock response to log_request".to_string()))
        // request::handle_log_request(agent_id, request, request_signature)
    }

    #[zome_fn("hc_public")]
    fn get_request(address: Address) -> ZomeApiResult<request::ClientRequestMeta> {
        request::handle_get_request(address)
    }

    #[zome_fn("hc_public")]
    fn log_response(
        request_commit: Address,
        response_hash: validate::Digest,
        host_metrics: response::HostMetrics,
        entries: Vec<response::HostEntryMeta>,
    ) -> ZomeApiResult<Address> {
        Ok(Address::from("Mock response to log_response".to_string()))
        // response::handle_log_response(request_commit, response_hash, host_metrics, entries)
    }

    #[zome_fn("hc_public")]
    fn get_response(address: Address) -> ZomeApiResult<response::HostResponseMeta> {
        response::handle_get_response(address)
    }

    #[zome_fn("hc_public")]
    fn log_service(
        agent_id: Address,
        response_commit: Address,
        confirmation: servicelog::Confirmation,
        confirmation_signature: validate::AgentSignature,
    ) -> ZomeApiResult<Address> {
        Ok(Address::from("Mock response to log_service".to_string()))
        // servicelog::handle_log_service(
        //     agent_id,
        //     response_commit,
        //     confirmation,
        //     confirmation_signature,
        // )
    }

    #[zome_fn("hc_public")]
    fn get_service(address: Address) -> ZomeApiResult<servicelog::ServiceLogMeta> {
        servicelog::handle_get_service(address)
    }

    #[zome_fn("hc_public")]
    fn list_uninvoiced_servicelogs() -> ZomeApiResult<Vec<Address>> {
        servicelog::handle_list_uninvoiced_servicelogs()
    }

    #[zome_fn("hc_public")]
    fn generate_invoice() -> ZomeApiResult<Option<Address>> {
        invoice::handle_generate_invoice()
    }

    #[zome_fn("hc_public")]
    fn list_unpaid_invoices() -> ZomeApiResult<Vec<Address>> {
        invoice::handle_list_unpaid_invoices()
    }

    #[zome_fn("hc_public")]
    fn get_payment_status() -> ZomeApiResult<invoice::PaymentStatus> {
        invoice::handle_get_payment_status()
    }

    #[zome_fn("hc_public")]
    fn get_traffic(filter: servicelog::TrafficFilter) -> ZomeApiResult<servicelog::TrafficGraph> {
        servicelog::handle_get_traffic(filter)
    }
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, PartialEq)]
pub struct WhoamiResult {
    pub hdk_version: String,
    pub hdk_hash: String,
    pub dna_address: String,
    pub dna_name: String,
    pub agent_id: AgentId,
    pub agent_address: String,
}

pub fn whoami_internal() -> ZomeApiResult<WhoamiResult> {
    Ok(WhoamiResult {
        hdk_version: hdk::version()?,
        hdk_hash: hdk::version_hash()?,
        dna_name: DNA_NAME.to_string(),
        dna_address: DNA_ADDRESS.to_string(),
        agent_id: JsonString::from_json(&AGENT_ID_STR).try_into()?,
        agent_address: AGENT_ADDRESS.to_string(),
    })
}

/// whoami_handler -- Return details of the local Agent or (if specified) some other Agent
pub fn whoami_handler() -> ZomeApiResult<WhoamiResult> {
    whoami_internal()
}
