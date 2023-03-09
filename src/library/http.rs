use polywrap_wasm_rs::{BigInt, JSON};
use crate::library::relay_call::{RelayCall, relay_call_path};
use crate::wrap::{HttpModule, HttpRequest, HttpResponseType, RelayResponse, TransactionStatusResponse};
use crate::wrap::imported::{ArgsGet, ArgsPost};
use crate::library::constants::GELATO_RELAY_URL;
use serde::{Deserialize};

#[derive(Deserialize)]
struct EstimatedFeeResponseContainer {
    estimated_fee: String,
}

#[derive(Deserialize)]
struct TransactionStatusResponseContainer {
    task: TransactionStatusResponse,
}

pub fn post_relay(relay_call: RelayCall, data: &JSON::Value) -> Result<RelayResponse, String> {
    let http_request = HttpRequest {
        headers: None,
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
        Some(data) => JSON::from_str::<RelayResponse>(&data).unwrap(),
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
        Some(data) => JSON::from_str::<EstimatedFeeResponseContainer>(&data).unwrap().estimated_fee,
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
    let transaction_status_response: TransactionStatusResponse = match response_body {
        Some(data) => JSON::from_str::<TransactionStatusResponseContainer>(&data).unwrap().task,
        None => return Ok(None),
    };
    Ok(Some(transaction_status_response))
}