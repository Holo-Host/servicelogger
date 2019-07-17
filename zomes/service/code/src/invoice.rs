#[allow(unused_imports)]
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    holochain_persistence_api::{
        hash::HashString,
        cas::content::{
            Address,
        },
    },
    holochain_json_api::{
        json::JsonString, error::JsonError,
    },
    holochain_core_types::{
        entry::Entry,
        error::HolochainError,
        dna::entry_types::Sharing,
        validation::EntryAction,
        validation::EntryValidationData,
        time::Iso8601
    },
    holochain_wasm_utils::api_serialization::{
        query::{
            QueryArgsOptions, QueryResult,
        },
    },
    AGENT_ADDRESS, AGENT_ID_STR, DNA_ADDRESS, DNA_NAME, PUBLIC_TOKEN,
};
// use std::convert::TryFrom;
use std::convert::{ TryInto, TryFrom };
// use serde::Serialize;
use serde_json::{self, json};

use super::servicelog;
use super::setup;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub enum HostingSituation {
    Hosting,
    Stopped
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct InvoicedLogs {
    pub servicelog_list: Vec<HashString>,
    pub holofuel_request: HashString,
    pub invoice_value: u64,
    pub last_invoiced_log: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct PaymentPref {
    pub provider_address: Address,
    pub dna_bundle_hash: HashString,
    pub max_fuel_per_invoice: f64,
    pub max_unpaid_value: f64,
    pub price_per_unit: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct PaymentStatus {
    pub unpaid_value: f64,
    pub max_unpaid_value: f64,
    pub max_fuel_per_invoice: f64,
    pub situation: HostingSituation,
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

fn is_unpaid(_invoiced_logs: &InvoicedLogs) -> bool {
    // Should bridge to Holofuel to get payment status
    true
}

fn list_unpaid_invoices() -> Result<Vec<(Address, InvoicedLogs)>, HolochainError> {
    let addr_entry_vec = match hdk::query_result(
        "invoiced_logs".into(),
        QueryArgsOptions{ entries: true, ..Default::default() }
    )? {
        QueryResult::Entries(addr_entry_vec) => Ok(addr_entry_vec),
        _ => Err(HolochainError::ErrorGeneric(format!("Unexpected hdk::query response for invoiced_logs"))),
    }?;

    addr_entry_vec
        .iter()
        .map(|(addr, entry)| {
            match entry {
                Entry::App(_entry_type, entry_value)
                    => Ok((addr.to_owned(), InvoicedLogs::try_from(entry_value)?)),
                unknown
                    => Err(HolochainError::ErrorGeneric(format!(
                        "Unexpected hdk::query response entry type for invoiced_logs: {:?}", &unknown))),
            }
        })
        .filter(|a_il_maybe| {
            match a_il_maybe {
                Ok((_addr, inv_log)) => is_unpaid(&inv_log),
                Err(_) => true,
            }
        })
        .collect::<Result<Vec<(Address, InvoicedLogs)>, HolochainError>>()
}

fn get_unpaid_value() -> Result<f64, HolochainError> {
    let invoices = list_unpaid_invoices()?;
    let mut value: f64 = 0.0;

    for (_addr, inv_logs) in invoices {
        value += inv_logs.invoice_value as f64;
    }

    return Ok(value);
}

pub fn handle_list_unpaid_invoices() -> ZomeApiResult<Vec<Address>> {
    Ok(list_unpaid_invoices()?
       .iter()
       .map(|(addr, _inv_log)| addr.to_owned())
       .collect())
}

pub fn handle_get_payment_status() -> ZomeApiResult<PaymentStatus> {
    // Bridge to Hosting App to get standard values
    let prefs  = get_payment_prefs()?;
    let unpaid_value = get_unpaid_value()?;
    let mut situation = HostingSituation::Hosting;

    if unpaid_value >= prefs.max_unpaid_value {
        situation = HostingSituation::Stopped;
    }

    Ok(PaymentStatus{
        unpaid_value,
        max_unpaid_value: prefs.max_unpaid_value,
        max_fuel_per_invoice: prefs.max_fuel_per_invoice,
        situation,
    }.into())
}

pub fn get_payment_prefs() -> ZomeApiResult<PaymentPref> {
    //** First get the latest configured dna_bundle_hash
    let dna_bundle_hash = match setup::get_latest_prefs() {
        Some(prefs) => prefs.dna_bundle_hash,
        None => return Err(ZomeApiError::Internal("DNA Bundle hash not configured!".to_string()))
    };

    // hdk::debug(format!("********DEBUG******** BRIDGING ready to call hosting-bridge for {:?}", dna_bundle_hash))?;
    let raw = hdk::call(
        "hosting-bridge",
        "host",
        Address::from(PUBLIC_TOKEN.to_string()),
        "get_service_log_details",
        json!({
            "app_hash": dna_bundle_hash,
        }).into()
    )?;
    // hdk::debug(format!("********DEBUG******** BRIDGING RAW response from hosting-bridge {:?}", raw))?;

    let prefs : PaymentPref = raw.try_into()?;

    Ok(prefs)
}

pub fn get_latest_invoice() -> Option<InvoicedLogs> {
    // Search the local chain for all InvoicedLogs
    let invoices_list: Vec<Address> = match hdk::query("invoiced_logs".into(), 0, 0) {
        Ok(results) => results,
        _ => vec![],
    };

    match invoices_list.last() {
        Some(last) => Some(hdk::utils::get_as_type(last.to_string().into()).unwrap()),
        None => None
    }

}

pub fn calculate_invoice_price(logs_list: &Vec<Address>) -> f64 {
    return 1.0 * logs_list.len() as f64;
}

pub fn handle_generate_invoice() -> ZomeApiResult<Option<Address>> {
    // Check payment prefs via bridge to Hosting app, and see if needed to generate an invoice automatically
    let prefs = get_payment_prefs()?;
    // Gets uninvoiced logs
    let logs_list = servicelog::list_uninvoiced_servicelogs();
    // TODO: calculate real invoice price
    let outstanding_value = calculate_invoice_price(&logs_list);

    // If not enough outstanding value to generate an invoice, return None
    if outstanding_value < prefs.max_fuel_per_invoice {
        return Ok(None);
    }

    // Otherwise generate the invoice
    let holofuel_address_raw = hdk::call(
        "holofuel-bridge",
        "transactions",
        Address::from(PUBLIC_TOKEN.to_string()),
        "request",
        json!({
            "from": prefs.provider_address,
            "amount": outstanding_value.to_string(),
            "notes": "service log", // TODO: put some nice notes
            "deadline": Iso8601::from(0) // TODO: use some actual dealine
        }).into()
    )?;

    // hdk::debug(format!("********DEBUG******** BRIDGING RAW response from fuel-bridge {:?}", holofuel_address_raw))?;

    let holofuel_address : Address = holofuel_address_raw.try_into()?;

    // hdk::debug(format!("********DEBUG******** BRIDGING ACTUAL response from fuel-bridge {:?}", holofuel_address))?;

    let entry = Entry::App("invoiced_logs".into(), InvoicedLogs{
            servicelog_list: logs_list,
            holofuel_request: holofuel_address,
            invoice_value: outstanding_value as u64,
            last_invoiced_log: 0,
    }.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(Some(address))
}

