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

    http::post_relay(RelayCall::CallWithSyncFee, &data).unwrap()
}

pub fn sponsored_call(args: ArgsSponsoredCall) -> RelayResponse {
    let is_supported = network::is_network_supported(&args.request.chain_id);
    match is_supported {
        Ok(false) => panic!("`Chain id [{}] is not supported`", args.request.chain_id.to_string()),
        Err(e) => panic!("{}", e),
        _ => {}
    }

    let mut data: JSON::Value = JSON::json!({
        "sponsorApiKey": args.sponsor_api_key,
        "chainId": args.request.chain_id,
        "target": address::get_checksum_address(&args.request.target),
        "data": args.request.data,
    });

    if let Some(options) = args.options {
        data["gasLimit"] = JSON::json!(options.gas_limit);
        data["retries"] = JSON::json!(options.retries);
    }

    http::post_relay(RelayCall::SponsoredCall, &data).unwrap()
}

pub fn get_estimated_fee(args: ArgsGetEstimatedFee) -> BigInt {
    let params: JSON::Value = JSON::json!({
        "paymentToken": args.payment_token,
        "gasLimit": args.gas_limit,
        "isHighPriority": args.is_high_priority,
        "gasLimitL1": args.gas_limit_l1.unwrap_or(BigInt::from(0)),
    });
    let data: JSON::Value = JSON::json!({
       "params": params
    });

    http::get_estimate(&args.chain_id, &data).unwrap()
}

pub fn get_task_status(args: ArgsGetTaskStatus) -> Option<TransactionStatusResponse> {
    http::get_task_status(&args.task_id).unwrap()
}