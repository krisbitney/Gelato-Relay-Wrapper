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

    let chain_id = u64::from_str_radix(args.request.chain_id.to_string().as_str(), 10).unwrap();
    let hex_data = format!("0x{}", hex::encode(args.request.data));

    let mut data: JSON::Value = JSON::json!({
        "chainId": chain_id,
        "target": args.request.target,
        "data": hex_data,
        "feeToken": args.request.fee_token,
        "isRelayContext": args.request.is_relay_context,
    });

    if let Some(options) = args.options {
        if let Some(gas_limit) = options.gas_limit {
            data["gasLimit"] = JSON::json!(gas_limit.to_string());
        }
        if let Some(retries) = options.retries {
            data["retries"] = JSON::json!(retries);
        }
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

    let chain_id = u64::from_str_radix(args.request.chain_id.to_string().as_str(), 10).unwrap();

    let mut data: JSON::Value = JSON::json!({
        "sponsorApiKey": args.sponsor_api_key,
        "chainId": chain_id,
        "target": args.request.target.to_string(),
        "data": args.request.data.to_string(),
    });

    if let Some(options) = args.options {
        if let Some(gas_limit) = options.gas_limit {
            let gas_limit_number = u64::from_str_radix(gas_limit.to_string().as_str(), 10).unwrap();
            data["gasLimit"] = gas_limit_number.to_string().into();
        }
        if let Some(retries) = options.retries {
            data["retries"] = retries.into();
        }
    }

    http::post_relay(RelayCall::SponsoredCall, &data).unwrap()
}

pub fn get_estimated_fee(args: ArgsGetEstimatedFee) -> BigInt {
    let gas_limit = u64::from_str_radix(args.gas_limit.to_string().as_str(), 10).unwrap();
    let gas_limit_l1_unwrapped = args.gas_limit_l1.unwrap_or(BigInt::from(0));
    let gas_limit_l1 = u64::from_str_radix(gas_limit_l1_unwrapped.to_string().as_str(), 10).unwrap();

    let params: JSON::Value = JSON::json!({
        "paymentToken": args.payment_token,
        "gasLimit": gas_limit,
        "isHighPriority": args.is_high_priority,
        "gasLimitL1": gas_limit_l1,
    });
    let data: JSON::Value = JSON::json!({
       "params": params
    });

    http::get_estimate(&args.chain_id, &data).unwrap()
}

pub fn get_task_status(args: ArgsGetTaskStatus) -> Option<TransactionStatusResponse> {
    http::get_task_status(&args.task_id).unwrap()
}