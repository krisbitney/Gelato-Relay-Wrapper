use polywrap_wasm_rs::JSON;
use crate::library::relay_call::{RelayCall, relay_call_path};
use crate::wrap::{HttpModule, HttpRequest, HttpResponseType, RelayResponse};
use crate::wrap::imported::ArgsPost;

pub fn post_relay(relay_call: RelayCall, http_request: HttpRequest) -> Result<RelayResponse, String> {
    let result = HttpModule::post(&ArgsPost {
        url: relay_call_path(&relay_call),
        request: Some(http_request)
    });
    let response_body = match result {
        Ok(Some(response)) => response.body,
        Ok(None) => return Err(format!("GelatoRelayWrapper/post: {} Failed with error: No data returned", relay_call)),
        Err(e) => return Err(format!("GelatoRelayWrapper/post: {} Failed with error: {}", relay_call, e)),
    };
    let relay_response: RelayResponse = match response_body {
        Some(data) => JSON::from_str::<RelayResponse>(&data).unwrap(),
        None => return Err(format!("GelatoRelayWrapper/post: {} Failed with error: No data returned", relay_call)),
    };
    Ok(relay_response)
}