use std::collections::HashMap;
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use crate::database::SyncAccount;

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct SyncHash {
    id: i32,
    syncHash: String,
}

#[derive(Serialize, Deserialize)]
struct ManifestResponse {
    version: i8,
    data: Vec<SyncHash>
}

#[derive(Debug, Clone)]
pub struct ResponseError {
    status: String,
    message: String,
}

impl ResponseError {
    pub fn formatted_message(&self) -> String {
        format!("Error {} {}", self.status, self.message)
    }
}

pub async fn get_jwt_token(base_url: &str, username: &str, password: &str) -> Result<String, ResponseError> {
    let url = format!("{}/api/login_check", base_url);
    let mut body = HashMap::new();
    body.insert("username", username);
    body.insert("password", password);

    let response = match make_post(url, body).await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let token_response: TokenResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    Ok(token_response.token)
}

async fn get_manifest(account: SyncAccount) -> Result<Vec<SyncHash>, ResponseError> {
    let url = format!("{}/api/records/hashes", account.url);

    let response = match make_get(url).await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let manifest_response: ManifestResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    Ok(manifest_response.data)
}

async fn make_get(url: String) -> Result<Response, ResponseError> {
    let client = reqwest::Client::builder();
    let res = client
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get(url)
        .send()
        .await;

    return match handle_response(res) {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    };
}


async fn make_post(url: String, body: HashMap<&str, &str>) -> Result<Response, ResponseError> {
    let client = reqwest::Client::builder();
    let res = client
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .post(url)
        .json(&body)
        .send()
        .await;

    return match handle_response(res) {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    };
}

fn handle_response(response: Result<Response, Error>) -> Result<Response, ResponseError>
{
    let valid_response = match response {
        Ok(res) => res,
        Err(e) => return Err(handle_reqwest_error(e)),
    };

    if !valid_response.status().is_success() {
        let error = ResponseError {
            status: valid_response.status().to_string(),
            message: "Error from server".to_string()
        };

        return Err(error)
    }

    return Ok(valid_response)
}

fn handle_reqwest_error(e: Error) -> ResponseError {
    let status = match e.status() {
        Some(status) => status.to_string(),
        _ => "0".to_string(),
    };
    let mut message: String = "Unknown Error".to_string();

    if e.is_redirect() {
        if let Some(final_stop) = e.url() {
            message = format!("redirect loop at: {}", final_stop);
        }
    }

    if e.is_body() {
        message = "Issue with request body.".to_string();
    }

    if e.is_connect() {
        message = "Connection could not be made.".to_string();
    }

    if e.is_request() {
        message = "Issue making the request to server".to_string();
    }

    if e.is_builder() {
        message = "FATAL ERROR: Builder Issue in client".to_string();
    }

    return ResponseError {
        status,
        message
    };
}