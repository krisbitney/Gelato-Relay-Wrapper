mod wrap;
mod library;

use polywrap_wasm_rs::{BigInt, JSON};
use wrap::*;
use library::*;
use crate::library::relay_call::RelayCall;

pub fn call_with_sync_fee(args: ArgsCallWithSyncFee) -> RelayResponse {
    let is_supported = network::is_network_supported(&args.request.chain_id);
    match is_supported {
        Ok(false) => panic!("`Chain id [{}] is not supported`", args.request.chain_id.to_string()),
        Err(e) => panic!("{}", e),
        _ => {}
    }

    let mut data: JSON::Value = JSON::json!({
        "chainId": args.request.chain_id,
        "target": args.request.target,
        "data": args.request.data,
        "feeToken": args.request.fee_token,
        "isRelayContext": args.request.is_relay_context,
    });

    if let Some(options) = args.options {
        data["gasLimit"] = JSON::json!(options.gas_limit);
        data["retries"] = JSON::json!(options.retries);
    }

    let http_request = HttpRequest {
        headers: None,
        url_params: None,
        response_type: HttpResponseType::TEXT,
        body: Some(data.to_string()),
        form_data: None,
        timeout: None,
    };

    http::post_relay(RelayCall::CallWithSyncFee, http_request).unwrap()
}

pub fn sponsored_call(args: ArgsSponsoredCall) -> RelayResponse {
    RelayResponse { task_id: String::from("") }
}

pub fn get_estimated_fee(args: ArgsGetEstimatedFee) -> BigInt {
    BigInt::from(0)
}

pub fn get_task_status(args: ArgsGetTaskStatus) -> Option<TransactionStatusResponse> {
    None
}