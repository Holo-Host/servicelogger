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
// use serde_json::{self, json};

use super::setup;

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
            // TODO: validate if payment_prefs is set
            // TODO: validate is the response exists and signature is valid
            validate_service_log(_my_entry, _validation_data)
        }
    )
}

fn validate_service_log(entry: ServiceLog, context: hdk::ValidationData) -> Result <(), String> {
    if setup::get_latest_payment_prefs().is_none() {
        return Err("Payment prefs not set, please perform setup prior to creating other entries".to_string())
    }

    match context.action {
        EntryAction::Create => match entry {
            ServiceLog { response_hash: hash, .. } => match hdk::get_entry(&hash) {
                Ok(maybe_entry) => match maybe_entry {
                    Some(_) => Ok(()),
                    None => Err("HostResponse entry not found!".to_string())
                }
                Err(e) => Err(e.to_string())
            },
        },
        _ => Err(format!("Invalid action for {:?}", entry)),
    }
}

pub fn handle_log_service(entry: ServiceLog) -> ZomeApiResult<Address> {
    let entry = Entry::App("service_log".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_service(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

pub fn list_servicelogs() -> Vec<Address> {
    match hdk::query("service_log".into(), 0, 0) {
        Ok(results) => results,
        _ => vec![],
    }
}

pub fn handle_list_servicelogs() -> Vec<Address> {
    list_servicelogs()
}

