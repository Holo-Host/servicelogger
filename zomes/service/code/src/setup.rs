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
        cas::content::Address
    },
};
//use serde::Serialize;
//use serde_json::{self, Value};


#[derive(Serialize, Deserialize, DefaultJson, Debug)]
pub struct PaymentPrefs {
    pub provider_address: Address,
    pub dna_bundle_hash: HashString,
    pub max_fuel_per_invoice: f64,
    pub max_unpaid_value: f64,
}

// TODO: maybe have an Anchor to point to the latest prefs object?
pub fn payment_prefs_definition() -> ValidatingEntryType {
    entry!(
        name: "payment_prefs",
        description: "the payment preferences defintion",
        sharing: Sharing::Public,
        native_type: PaymentPrefs,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_my_entry: PaymentPrefs, _validation_data: hdk::ValidationData| {
            Ok(())
        }
    )
}

pub fn handle_set_payment_prefs(entry: PaymentPrefs) -> ZomeApiResult<Address> {
    let entry = Entry::App("payment_prefs".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn get_latest_payment_prefs() -> Option<PaymentPrefs> {
    // Search the local chain for all payment_prefs
    let prefs_list: Vec<Entry> = match hdk::query("payment_prefs".into(), 0, 0) {
        Ok(results) => results,
        _ => vec![],
    }.iter().map(|address| {
        hdk::get_entry(&address).unwrap().unwrap()
    }).collect();

    prefs_list.last()
    .map(|entry| {
        let json = match entry {
            Entry::App(_, entry_value) => entry_value.into(),
            _ => "null".into()
        };
        serde_json::from_str(json).unwrap()
    })
}
