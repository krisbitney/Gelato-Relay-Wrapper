mod library;
mod wrap;

use crate::library::relay_call::RelayCall;
use library::*;
use polywrap_msgpack_serde::BigIntWrapper;
use polywrap_wasm_rs::{BigInt, JSON};
use wrap::*;

impl ModuleTrait for Module {
    fn call_with_sync_fee(args: ArgsCallWithSyncFee) -> Result<RelayResponse, String> {
        let is_supported = network::is_network_supported(&args.request.chain_id);
        match is_supported {
            Ok(false) => panic!(
                "`Chain id [{}] is not supported`",
                args.request.chain_id.0.to_string()
            ),
            Err(e) => panic!("{}", e),
            _ => {}
        }

        let chain_id =
            u64::from_str_radix(args.request.chain_id.0.to_string().as_str(), 10).unwrap();

        let mut data: JSON::Value = JSON::json!({
            "chainId": chain_id,
            "target": args.request.target,
            "data": args.request.data,
            "feeToken": args.request.fee_token,
            "isRelayContext": args.request.is_relay_context,
        });

        if let Some(options) = args.options {
            if let Some(gas_limit) = options.gas_limit {
                let gas_limit_number =
                    u64::from_str_radix(gas_limit.0.to_string().as_str(), 10).unwrap();
                data["gasLimit"] = gas_limit_number.to_string().into();
            }
            if let Some(retries) = options.retries {
                data["retries"] = retries.into();
            }
        }

        http::post_relay(RelayCall::CallWithSyncFee, &data)
            .map_err(|e| format!("Error executing post_relay: {e}"))
    }

    fn sponsored_call(args: ArgsSponsoredCall) -> Result<RelayResponse, String> {
        let is_supported = network::is_network_supported(&args.request.chain_id);
        match is_supported {
            Ok(false) => panic!(
                "`Chain id [{}] is not supported`",
                args.request.chain_id.0.to_string()
            ),
            Err(e) => panic!("{}", e),
            _ => {}
        }

        let chain_id =
            u64::from_str_radix(args.request.chain_id.0.to_string().as_str(), 10).unwrap();

        let mut data: JSON::Value = JSON::json!({
            "sponsorApiKey": args.sponsor_api_key,
            "chainId": chain_id,
            "target": args.request.target.to_string(),
            "data": args.request.data.to_string(),
        });

        if let Some(options) = args.options {
            if let Some(gas_limit) = options.gas_limit {
                let gas_limit_number =
                    u64::from_str_radix(gas_limit.0.to_string().as_str(), 10).unwrap();
                data["gasLimit"] = gas_limit_number.to_string().into();
            }
            if let Some(retries) = options.retries {
                data["retries"] = retries.into();
            }
        }

        http::post_relay(RelayCall::SponsoredCall, &data)
            .map_err(|e| format!("Error executing post_relay: {e}"))
    }

    fn get_estimated_fee(args: ArgsGetEstimatedFee) -> Result<BigIntWrapper, String> {
        let gas_limit = u64::from_str_radix(args.gas_limit.0.to_string().as_str(), 10).unwrap();
        let gas_limit_l1_unwrapped = args.gas_limit_l1.unwrap_or(BigIntWrapper(BigInt::from(0)));
        let gas_limit_l1 =
            u64::from_str_radix(gas_limit_l1_unwrapped.0.to_string().as_str(), 10).unwrap();

        let data: JSON::Value = JSON::json!({
            "paymentToken": args.payment_token,
            "gasLimit": gas_limit.to_string(),
            "isHighPriority": args.is_high_priority.to_string(),
            "gasLimitL1": gas_limit_l1.to_string(),
        });

        http::get_estimate(&args.chain_id, &data)
            .map_err(|e| format!("Error executing get_estimate: {e}"))
    }

    fn get_task_status(
        args: ArgsGetTaskStatus,
    ) -> Result<Option<TransactionStatusResponse>, String> {
        http::get_task_status(&args.task_id)
            .map_err(|e| format!("Error executing get_estimate: {e}"))
    }
}
