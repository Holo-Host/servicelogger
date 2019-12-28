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
use std::convert::TryFrom;

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

/// Get the last configured SetupPrefs, if any
pub fn get_latest_prefs() -> Option<SetupPrefs> {
    if let Ok(prefs) = hdk::query("setup_prefs".into(), 0, 0) {
        if let Some(address) = prefs.last() {
            if let Ok(Some(entry)) = hdk::get_entry(&address) {
                if let Entry::App(_, entry_value) = entry {
                    if let Ok(value) = SetupPrefs::try_from(entry_value.to_owned()) {
                        return Some(value)
                    }
                }
            }
        }
    }
    None
}
