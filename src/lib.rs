mod wrap;
mod library;

use polywrap_wasm_rs::BigInt;
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

    // TODO: need to create http request

    // type RelayRequestOptions {
    //     gasLimit: BigInt
    //     retries: Int
    // }
    //
    // type CallWithSyncFeeRequest {
    //     chainId: BigInt!
    //     target: String!
    //     data: Bytes!
    //     feeToken: String!
    //     isRelayContext: Boolean // defaults to true
    // }

    let http_request = HttpRequest {
        headers: None,
        url_params: None,
        response_type: HttpResponseType::TEXT,
        body: None,
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