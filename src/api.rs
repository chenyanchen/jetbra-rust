use anyhow::Result;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::app::App;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetActiveCodeRequest {
    token: String,
    serial_number: String,
}

impl GetActiveCodeRequest {
    pub fn new(token: String, serial_number: String) -> Self {
        Self {
            token,
            serial_number,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetActiveCodeResponse {
    pub apps: Vec<App>,
}

pub async fn get_active_code(req: &GetActiveCodeRequest) -> Result<GetActiveCodeResponse> {
    // Create a custom client with a DangerousClientConfig that disables certificate verification
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    // Make an insecure HTTPS request
    let response = client
        .post("https://120.27.245.86/api/v1/kit/active-code")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(req)?)
        .send()
        .await?;

    if response.status() != StatusCode::OK {
        return Err(anyhow::anyhow!("Status code is {}", response.status()));
    }

    // let resp: GetActiveCodeResponse = response.json().await?;
    let resp: GetActiveCodeResponse = response.json().await?;

    Ok(resp)
}
