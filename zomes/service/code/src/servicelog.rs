
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
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
        dna::entry_types::Sharing,
        validation::EntryValidationData
    },
};
use crate::validate::*;
use super::request;
use super::response;
use super::invoice;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct ServiceLog {
    response_commit: Address,
    client_signature: AgentSignature, // signed response_hash
}

pub fn service_log_definition() -> ValidatingEntryType {
    entry!(
        name: "service_log",
        description: "this it the entry defintion for a service log",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |context: hdk::EntryValidationData<ServiceLog>| {
            validate_service_log(context)
        }
    )
}

fn validate_service_log(context: EntryValidationData<ServiceLog>) -> Result<(), String> {
    match context {
        EntryValidationData::Create{entry:obj,validation_data:_} => match obj {
            ServiceLog { response_commit: hash, .. } => match hdk::get_entry(&hash) {
                Ok(maybe_entry) => match maybe_entry {
                    Some(_) => Ok(()),
                    None => Err("HostResponse entry not found!".to_string())
                }
                Err(e) => Err(e.to_string())
            },
        }
        _ => {
            Err("Failed to validate with wrong entry type".to_string())
        }
    }
}

pub fn handle_log_service(entry: ServiceLog) -> ZomeApiResult<Address> {
    let entry = Entry::App("service_log".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;

    return Ok(address);
}

pub fn handle_get_service(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

fn _get_original_request(address: Address) -> ZomeApiResult<request::ClientRequest> {
    let log : ServiceLog = hdk::utils::get_as_type(address)?;
    let response : response::HostResponse = hdk::utils::get_as_type(log.response_commit)?;
    hdk::utils::get_as_type(response.request_commit)
}

pub fn list_uninvoiced_servicelogs() -> Vec<Address> {
    match handle_list_uninvoiced_servicelogs() {
        Ok(results) => results,
        _ => vec![],
    }
}

pub fn handle_list_uninvoiced_servicelogs() -> ZomeApiResult<Vec<Address>> {
    // List all InvoicedLogs, then join the list of all servicelog_list inside them
    let last_log = match invoice::get_latest_invoice() {
        Some(invoice) => invoice.last_invoiced_log,
        None => 0
    };

    hdk::query("service_log".into(), last_log, 0)
}

