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
        validation::EntryValidationData,
        time::Iso8601
    },
    AGENT_ADDRESS, AGENT_ID_STR, DNA_ADDRESS, DNA_NAME, PUBLIC_TOKEN,
};
// use serde::Serialize;
use serde_json::{self, json};

use super::servicelog;
use super::setup;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct InvoicedLogs {
    servicelog_list: Vec<HashString>,
    holofuel_request: HashString,
    invoice_value: u64,
}
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct PaymentPrefs {
    pub provider_address: Address,
    pub dna_bundle_hash: HashString,
    pub max_fuel_per_invoice: f64,
    pub max_unpaid_value: f64,
}

pub fn invoiced_logs_definition() -> ValidatingEntryType {
    entry!(
        name: "invoiced_logs",
        description: "this it the entry defintion for a bundle of invoiced service logs",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |context: hdk::EntryValidationData<InvoicedLogs>| {
            validate_invoiced_logs(context)
        }
    )
}

fn validate_invoiced_logs(context: hdk::EntryValidationData<InvoicedLogs>) -> Result <(), String> {
    match context {
        EntryValidationData::Create{entry:obj,validation_data:_} => match obj {
            InvoicedLogs { servicelog_list: hashes, .. } => hashes.iter().map(|hash| match hdk::get_entry(&hash) {
                Ok(maybe_entry) => match maybe_entry {
                    Some(_) => Ok(()),
                    None => Err("ServiceLog entry not found!".to_string())
                }
                Err(e) => Err(e.to_string())
            }).collect(),
        },
        _ => {
            Err("Failed to validate with wrong entry type".to_string())
        }
    }
}

pub fn handle_generate_invoice(price_per_unit: Option<u64>) -> ZomeApiResult<Address> {

    let dna_bundle_hash = match setup::get_latest_prefs() {
        Some(dna_bundle_hash) => dna_bundle_hash,
        None => return Err(ZomeApiError::Internal("DNA Bundle hash not configured!".to_string()))
    };

    hdk::debug(format!("********DEBUG******** BRIDGING ready to call hosting-bridge {:?}", &hdk::THIS_INSTANCE))?;
    let payment_prefs: PaymentPrefs = match hdk::call(
        "hosting-bridge",
        "provider",
        Address::from(PUBLIC_TOKEN.to_string()),
        "get_app_details",
        json!({
            "app_hash": dna_bundle_hash,
        }).into()
    ) {
        Ok(json) => serde_json::from_str(&json.to_string()).unwrap(),
        Err(e) => return Err(e)
    };

    let holofuel_address = match hdk::call(
        "holofuel-bridge",
        "transactions",
        Address::from(PUBLIC_TOKEN.to_string()),
        "request",
        json!({
            "from": payment_prefs.provider_address,
            "amount": "1", //price_per_unit.unwrap(), // TODO: use the real value
            "notes": "service log", // TODO: put some nice notes
            "deadline": Iso8601::from(0) // TODO: use some actual dealine
        }).into()
    ) {
        Ok(json) => serde_json::from_str(&json.to_string()).unwrap(),
        Err(e) => return Err(e)
    };

    let logs_list = servicelog::list_servicelogs();

    let entry = Entry::App("invoiced_logs".into(), InvoicedLogs{
            servicelog_list: logs_list,
            holofuel_request: holofuel_address,
            invoice_value: price_per_unit.unwrap(),
    }.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}
