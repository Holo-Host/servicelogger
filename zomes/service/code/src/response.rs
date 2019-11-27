
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
        validation::EntryValidationData,
        time::Period,
    },
};

use crate::validate::*; // AgentId, AgentSignature, Digest, ...


#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct HostResponse {
    pub request_commit: Address,
    pub response_hash: Digest,
    pub host_metrics: Metrics,
    pub entries: Vec<HostEntryMeta>
}

pub fn host_response_definition() -> ValidatingEntryType {
    entry!(
        name: "host_response",
        description: "this it the entry defintion for a host response",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |context: hdk::EntryValidationData<HostResponse>| {
            validate_response(context)
        }
    )
}

fn validate_response(context: EntryValidationData<HostResponse>) -> Result<(), String> {
    match context {
        EntryValidationData::Create{entry:obj,validation_data:_} => match obj {
            // Ensures that the response references an existing request
            HostResponse { request_commit: hash, .. } => match hdk::get_entry(&hash) {
                Ok(maybe_entry) => match maybe_entry {
                    Some(_) => Ok(()),
                    None => Err("ClientRequest entry not found!".to_string())
                }
                Err(e) => Err(e.to_string())
            },
        }
        _ => {
            Err("Failed to validate with wrong entry type".to_string())
        }
    }
}


/// Client Request metrics from the perspective of the Host
///
/// TODO: Add other envoy and holochain conductor collected metrics collected during the operation
/// of the conductor Zome API call, or the envoy-mediated collection of a Client signature on a
/// source-chain Entry.  Note that these are likely to be *estimates*; Linux collects such metrics
/// based on Threads, and the Node Envoy and Rust Conductor threads may service multiple Client
/// requests.  These stats can only be estimates in aggregate, over multiple blocks of Request
/// transactions (eg. ServiceLogger Invoices), using statistical methods (eg. linear regression over
/// many blocks of Requests, vs. the summation of all Envoy and Conductor (Zome + DHT thread)
/// activity.
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Metrics {
    pub cpu: Cpu,
    pub network: IO,
    pub storage: i64, // in theory, a Zome call could reduce net storage used
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Cpu {
    pub elapsed: Period,
    pub user: Period,
    pub system: Period,
    pub load: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct IO {
    pub i: u64,
    pub o: u64
}

/// Client Request Entry signing metadata and metrics from the perspective of the Host.
#[derive(Debug, Clone, DefaultJson, Serialize, Deserialize)]
pub struct HostEntryMeta {
    pub hash: Digest,
    pub hash_signature: AgentSignature,
    pub host_metrics: Metrics,
}

/// Commit a Hosting response report.  The `response_hash` is the Digest of the response sent back
/// to the client.  The client will sign a package including `response_commit` (this commit), and
/// the `response_hash` ( affirming that they received the same response claimed sent by the Host.
/// This is sufficient to gain proof from the Client of Hosting service rendered.  Only when we
/// `log_service` (and confirm that the signed `request_commit` and `response_hash` matched) will a
/// complete round-trip request-response be non-repudiably affirmed by the Client: "I asked for this,
/// got and affirm that I got the claimed response".
pub fn handle_log_response(
    request_commit: Address,
    response_hash: Digest,
    host_metrics: Metrics,
    entries: Vec<HostEntryMeta>
) -> ZomeApiResult<Address> {
    let entry = Entry::App(
        "host_response".into(),
        HostResponse { request_commit, response_hash, host_metrics, entries }.into()
    );
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

#[derive(Debug, Clone, DefaultJson, Serialize, Deserialize)]
pub struct HostResponseMeta {
    pub meta: CommitMeta,
    pub host_response: HostResponse,
}

pub fn handle_get_response(
    address: Address
) -> ZomeApiResult<HostResponseMeta> {
    let (meta,host_response) = get_meta_and_entry_as_type::<HostResponse>(address)?;
    Ok(HostResponseMeta { meta, host_response })

}
