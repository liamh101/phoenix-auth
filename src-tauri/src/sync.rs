use std::collections::HashMap;
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
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
        Err(e) => return Err(handle_reqwest_error(e)),
    };

    if !response.status().is_success() {
        let error = ResponseError {
            status: response.status().to_string(),
            message: "Error from server".to_string()
        };

        return Err(error)
    }

    let token_response: TokenResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    Ok(token_response.token)
}


async fn make_post(url: String, body: HashMap<&str, &str>) -> Result<Response, Error> {
    let client = reqwest::Client::builder();

    client
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .post(url)
        .json(&body)
        .send()
        .await
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