use crate::wrap::{HttpModule, HttpRequest, HttpResponseType};
use crate::wrap::imported::http_module::ArgsGet;
use polywrap_wasm_rs::{JSON, BigInt};
use crate::library::constants::GELATO_RELAY_URL;
use serde::{Deserialize};

#[derive(Deserialize)]
struct GetSupportedNetworksResponse {
    relays: Vec<String>,
}

pub fn is_network_supported(chain_id: &BigInt) -> Result<bool, String> {
    let supported_networks = get_supported_networks()?;
    Ok(supported_networks.contains(&chain_id.to_string()))
}

pub fn get_supported_networks() -> Result<Vec<String>, String> {
    let result = HttpModule::get(&ArgsGet {
        url: GELATO_RELAY_URL.to_string() + "/relays/v2",
        request: Some(HttpRequest {
            headers: None,
            url_params: None,
            response_type: HttpResponseType::TEXT,
            body: None,
            form_data: None,
            timeout: None,
        })
    });
    let response_body = match result {
        Ok(Some(response)) => response.body,
        Ok(None) => return Err("GelatoRelayWrapper/get_supported_networks: Failed with error: No data returned".to_string()),
        Err(e) => return Err(format!("GelatoRelayWrapper/get_supported_networks: Failed with error: {}", e)),
    };
    let supported_networks: Vec<String> = match response_body {
        Some(data) => JSON::from_str::<GetSupportedNetworksResponse>(&data).unwrap().relays,
        None => return Err("GelatoRelayWrapper/get_supported_networks: Failed with error: No data returned".to_string()),
    };
    Ok(supported_networks)
}