use polywrap_wasm_rs::{BigInt, JSON, Map};
use crate::library::relay_call::{RelayCall, relay_call_path};
use crate::wrap::{get_task_state_value, HttpModule, HttpRequest, HttpResponseType, RelayResponse, TransactionStatusResponse};
use crate::wrap::imported::{ArgsGet, ArgsPost};
use crate::library::constants::GELATO_RELAY_URL;
use serde::{Deserialize};

#[derive(Deserialize)]
struct EstimatedFeeResponse {
    #[serde(rename = "estimatedFee")]
    estimated_fee: String,
}

#[derive(Deserialize)]
struct TaskStatusResponse {
    task: TaskResponse,
}

#[derive(Deserialize)]
pub struct RelayedTransactionResponse {
    #[serde(rename = "taskId")]
    pub task_id: String,
}

impl From<RelayedTransactionResponse> for RelayResponse {
    fn from(to: RelayedTransactionResponse) -> Self {
        Self {
            task_id: to.task_id
        }
    }
}

#[derive(Deserialize)]
pub struct TaskResponse {
    #[serde(rename(deserialize = "chainId"))]
    pub chain_id: u64,
    #[serde(rename(deserialize = "taskId"))]
    pub task_id: String,
    #[serde(rename(deserialize = "taskState"))]
    pub task_state: String,
    #[serde(rename(deserialize = "creationDate"))]
    pub creation_date: String,
    #[serde(rename(deserialize = "lastCheckDate"))]
    pub last_check_date: Option<String>,
    #[serde(rename(deserialize = "lastCheckMessage"))]
    pub last_check_message: Option<String>,
    #[serde(rename(deserialize = "transactionHash"))]
    pub transaction_hash: Option<String>,
    #[serde(rename(deserialize = "blockNumber"))]
    pub block_number: Option<u64>,
    #[serde(rename(deserialize = "executionDate"))]
    pub execution_date: Option<String>,
}

pub fn post_relay(relay_call: RelayCall, data: &JSON::Value) -> Result<RelayResponse, String> {
    let mut headers = Map::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    let http_request = HttpRequest {
        headers: Some(headers),
        url_params: None,
        response_type: HttpResponseType::TEXT,
        body: Some(data.to_string()),
        form_data: None,
        timeout: None,
    };

    let result = HttpModule::post(&ArgsPost {
        url: relay_call_path(&relay_call),
        request: Some(http_request)
    });
    let response_body = match result {
        Ok(Some(response)) => response.body,
        Ok(None) => return Err(format!("GelatoRelayWrapper/post_relay: {} Failed with error: No data returned", relay_call)),
        Err(e) => return Err(format!("GelatoRelayWrapper/post_relay: {} Failed with error: {}", relay_call, e)),
    };
    let relay_response: RelayResponse = match response_body {
        Some(data) => {
            JSON::from_str::<RelayedTransactionResponse>(&data).unwrap().into()
        },
        None => return Err(format!("GelatoRelayWrapper/post_relay: {} Failed with error: No data returned", relay_call)),
    };
    Ok(relay_response)
}

pub fn get_estimate(chain_id: &BigInt, data: &JSON::Value) -> Result<BigInt, String> {
    let http_request = HttpRequest {
        headers: None,
        url_params: None,
        response_type: HttpResponseType::TEXT,
        body: Some(data.to_string()),
        form_data: None,
        timeout: None,
    };
    let result = HttpModule::get(&ArgsGet {
        url: format!("{}/oracles/{}/estimate", GELATO_RELAY_URL, chain_id.to_string()),
        request: Some(http_request)
    });
    let response_body = match result {
        Ok(Some(response)) => response.body,
        Ok(None) => return Err("GelatoRelayWrapper/get_estimate: Failed with error: No data returned".to_string()),
        Err(e) => return Err(format!("GelatoRelayWrapper/get_estimate: Failed with error: {}", e)),
    };
    let estimated_fee = match response_body {
        Some(data) => JSON::from_str::<EstimatedFeeResponse>(&data).unwrap().estimated_fee,
        None => return Err("GelatoRelayWrapper/get_estimate: Failed with error: No data returned".to_string()),
    };
    estimated_fee.parse::<BigInt>()
        .map_err(|e| format!("GelatoRelayWrapper/get_estimate: Failed with error: {}", e))
}

pub fn get_task_status(task_id: &str) -> Result<Option<TransactionStatusResponse>, String> {
    let result = HttpModule::get(&ArgsGet {
        url: format!("{}/tasks/status/{}", GELATO_RELAY_URL, task_id),
        request: None
    });
    let response_body = match result {
        Ok(Some(response)) => response.body,
        Ok(None) => return Err("GelatoRelayWrapper/get_task_status: Failed with error: No data returned".to_string()),
        Err(e) => return Err(format!("GelatoRelayWrapper/get_task_status: Failed with error: {}", e)),
    };

    let task_response: TaskResponse = match response_body.clone() {
        Some(data) => JSON::from_str::<TaskStatusResponse>(&data).unwrap().task,
        None => return Ok(None),
    };
    Ok(Some(
        TransactionStatusResponse {
            chain_id: BigInt::from(task_response.chain_id),
            task_id: task_response.task_id,
            task_state: get_task_state_value(&task_response.task_state).unwrap(),
            creation_date: task_response.creation_date,
            last_check_date: task_response.last_check_date,
            last_check_message: task_response.last_check_message,
            transaction_hash: task_response.transaction_hash,
            block_number: if task_response.block_number.is_some() {
                Some(BigInt::from(task_response.block_number.unwrap()))
            } else {
                None
            },
            execution_date: task_response.execution_date,
        }
    ))
}