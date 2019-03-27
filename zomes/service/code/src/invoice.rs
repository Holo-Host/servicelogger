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
    AGENT_ADDRESS, AGENT_ID_STR, DNA_ADDRESS, DNA_NAME, PUBLIC_TOKEN,
};
// use serde::Serialize;
use serde_json::{self, json};

use super::servicelog;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct InvoicedLogs {
    servicelog_list: Vec<HashString>,
    holofuel_request: HashString,
    invoice_value: u64,
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
    hdk::debug(format!("********DEBUG******** instance {:?}", &hdk::THIS_INSTANCE))?;

    let holofuel_address = match hdk::call(
        "random",
        "transactions",
        "".into(),
        "request",
        json!({
            "from": "xxx-undefined_address-xxx",
            "amount": price_per_unit.unwrap(), // TODO: use the real value
            "notes": "service log", // TODO: put some nice notes
            "deadline": "" // TODO: use some actual dealine
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
