use super::invoice;
use super::request;
use super::response;
use crate::validate::*;
use hdk::holochain_core_types::time::Period;
use hdk::prelude::*;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct ServiceLog {
    agent_id: Address,
    response_commit: Address,
    confirmation: Confirmation,
    confirmation_signature: AgentSignature, // signed response_hash
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
            // validate_service_log(context)
            Ok(())
        }
    )
}

// fn validate_service_log(context: EntryValidationData<ServiceLog>) -> Result<(), String> {
//     match context {
//         EntryValidationData::Create {
//             entry:
//                 ServiceLog {
//                     agent_id,
//                     response_commit,
//                     confirmation,
//                     confirmation_signature,
//                 },
//             validation_data: _,
//         } => {
//             // Ensures that the service_log references an existing response
//             let response_meta = response::handle_get_response(response_commit)
//                 .map_err(|_| String::from("HostResponse entry not found!"))?;
//             // Ensures that the service_log referenced the agreed response_hash
//             if response_meta.host_response.response_hash != confirmation.response_hash {
//                 return Err(String::from("ServiceLog.response_hash didn't match"));
//             }
//             // Ensure Client agent_id actually signed confirmation
//             let confirmation_serialization =
//                 serde_json::to_string(&confirmation).map_err(|e| e.to_string())?;
//             if !agent_id.verify(
//                 &confirmation_serialization.as_bytes(),
//                 &confirmation_signature,
//             ) {
//                 return Err(format!(
//                     "Signature {} invalid for service_log: {}",
//                     &confirmation_signature, &confirmation_serialization
//                 ));
//             };
//             Ok(())
//         }
//         _ => Err(String::from("Failed to validate with wrong entry type")),
//     }
// }

#[derive(Debug, Clone, DefaultJson, Serialize, Deserialize)]
pub struct Confirmation {
    client_metrics: ClientMetrics,
    response_hash: Digest,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct ClientMetrics {
    pub duration: Period,
    // TODO: What other Client-side metrics can Chaperone collect?
}

pub fn handle_log_service(
    agent_id: Address,
    response_commit: Address,
    confirmation: Confirmation,
    confirmation_signature: AgentSignature,
) -> ZomeApiResult<Address> {
    let entry = Entry::App(
        "service_log".into(),
        ServiceLog {
            agent_id,
            response_commit,
            confirmation,
            confirmation_signature,
        }
        .into(),
    );
    // let address = hdk::commit_entry(&entry)?;
    // Ok(address)
    hdk::entry_address(&entry)
}

#[derive(Debug, Clone, DefaultJson, Serialize, Deserialize)]
pub struct ServiceLogMeta {
    pub meta: CommitMeta,
    pub service_log: ServiceLog,
}

pub fn handle_get_service(address: Address) -> ZomeApiResult<ServiceLogMeta> {
    let (meta, service_log) = get_meta_and_entry_as_type::<ServiceLog>(address)?;
    Ok(ServiceLogMeta { meta, service_log })
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct TrafficGraph {
    start_date: Option<Iso8601>,
    total_zome_calls: u64,
    value: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub enum TrafficFilter {
    YEAR,
    MONTH,
    WEEK,
    DAY,
    HOUR,
    SECOND,
}

pub fn handle_get_traffic(filter: TrafficFilter) -> ZomeApiResult<TrafficGraph> {
    let mut date_array = handle_get_service_log()?;
    date_array.sort();
    // match YEARS/MONTH/DAY/HOUR
    let period = match filter {
        TrafficFilter::YEAR => Period::try_from("1y").unwrap(),
        TrafficFilter::MONTH => Period::try_from("1m").unwrap(),
        TrafficFilter::WEEK => Period::try_from("1w").unwrap(),
        TrafficFilter::DAY => Period::try_from("1d").unwrap(),
        TrafficFilter::HOUR => Period::try_from("1h").unwrap(),
        TrafficFilter::SECOND => Period::try_from("1s").unwrap(),
    };
    if !date_array.is_empty() {
        let mut value: Vec<u64> = Vec::new();
        let mut x = 0;
        let mut index = 0;
        let mut initial_date = date_array[0].to_owned().clone();
        for date in &date_array {
            let check_date = initial_date + period.clone();
            if date > &check_date?.to_owned() {
                value.push(x);
                // reset
                x = 1;
                initial_date = date_array[index].to_owned();
            } else {
                x += 1;
            }
            index += 1;
        }
        value.push(x);

        Ok(TrafficGraph {
            start_date: Some(date_array[0].to_owned().clone()),
            total_zome_calls: value.clone().iter().sum(),
            value,
        })
    } else {
        return Ok(TrafficGraph {
            start_date: None,
            total_zome_calls: 0,
            value: [].to_vec(),
        });
    }
}

fn handle_get_service_log() -> ZomeApiResult<Vec<Iso8601>> {
    match hdk::query_result(
        "service_log".into(),
        QueryArgsOptions {
            headers: true,
            ..Default::default()
        },
    )? {
        QueryResult::Headers(headers) => {
            let headers = headers.iter().map(|header| header.timestamp());
            let mut c = Vec::new();
            for i in headers {
                c.push(i.to_owned());
            }
            Ok(c)
        }
        _ => Ok([].to_vec()),
    }
}

fn _get_original_request(address: Address) -> ZomeApiResult<request::ClientRequest> {
    let log: ServiceLog = hdk::utils::get_as_type(address)?;
    let response: response::HostResponse = hdk::utils::get_as_type(log.response_commit)?;
    hdk::utils::get_as_type(response.request_commit)
}

pub fn handle_list_uninvoiced_servicelogs() -> ZomeApiResult<Vec<Address>> {
    list_servicelogs_since(invoice::get_latest_invoice().map(|invoice| invoice.last_invoiced_log))
}

pub fn list_servicelogs_since(last_log: Option<usize>) -> ZomeApiResult<Vec<Address>> {
    hdk::query("service_log".into(), last_log.unwrap_or(0), 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek;
    use std::convert::{From, TryInto};

    #[test]
    fn client_service_smoke() {
        // Get a legit request_hash signature, agent_id
        let secret: [u8; 32] = [0_u8; 32];
        let secret_key = ed25519_dalek::SecretKey::from_bytes(&secret).unwrap();
        let public_key = ed25519_dalek::PublicKey::from(&secret_key);
        let secret_key_exp = ed25519_dalek::ExpandedSecretKey::from(&secret_key);
        let agent_id = Agent::from(&secret_key);

        let service_log_str = format!(
            r#"{{
  "agent_id": "{}",
  "response_commit": "QmQTYWmk2rY5hQQECHMM8kcjxve3CdYMbXWs62fUP4HZMq",
  "confirmation": {{
    "client_metrics": {{
      "duration": "1.23s"
    }},
    "response_hash": "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aV1xv"
  }},
  "confirmation_signature": "XxHr36xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxCg=="
}}"#,
            agent_id
        );

        let mut service_log: ServiceLog = serde_json::from_str(&service_log_str).unwrap();
        assert_eq!(
            serde_json::to_string_pretty(&service_log).unwrap(),
            service_log_str
        );

        let signature = secret_key_exp.sign(
            serde_json::to_string(&service_log.confirmation)
                .unwrap()
                .as_bytes(),
            &public_key,
        );
        service_log.confirmation_signature =
            AgentSignature::from_bytes(&signature.to_bytes()).unwrap();
        println!(
            "ServiceLog 1 valid: {}",
            serde_json::to_string_pretty(&service_log).unwrap()
        );

        service_log.response_commit = "QmU84Rqgs2bzBDYsp2too1oR2HYnrG5KxAMYBkcrPzjJ5w".into();
        service_log.confirmation.response_hash = "QmVtcYog4isPhcurmZxkggnCnoKVdAmb97VZy6Th6aXyzv"
            .try_into()
            .unwrap();
        let signature = secret_key_exp.sign(
            serde_json::to_string(&service_log.confirmation)
                .unwrap()
                .as_bytes(),
            &public_key,
        );
        service_log.confirmation_signature =
            AgentSignature::from_bytes(&signature.to_bytes()).unwrap();
        println!(
            "ServiceLog 2 valid: {}",
            serde_json::to_string_pretty(&service_log).unwrap()
        );
    }
}
