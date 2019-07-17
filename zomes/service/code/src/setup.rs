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
        validation::EntryValidationData
    },
};
// use serde::Serialize;
use serde_json::{self};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct SetupPrefs {
    pub dna_bundle_hash: HashString 
}

pub fn setup_prefs_definition() -> ValidatingEntryType {
    entry!(
        name: "setup_prefs",
        description: "this contains the setup prefs set by the Interceptr",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<SetupPrefs>| {
            Ok(())
        }
    )
}

pub fn handle_setup(entry: SetupPrefs) -> ZomeApiResult<Address> {
    let entry = Entry::App("setup_prefs".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn get_latest_prefs() -> Option<SetupPrefs> {
    // Search the local chain for all setup_prefs
    let prefs_list: Vec<Entry> = match hdk::query("setup_prefs".into(), 0, 0) {
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
