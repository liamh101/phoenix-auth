use crate::database::{Account, AccountAlgorithm, SyncAccount};
use crate::encryption;
use reqwest::header::AUTHORIZATION;
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[cfg(test)]
#[path = "./sync_api_test.rs"]
mod tests;

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncManifest {
    pub id: i32,
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize)]
struct ManifestResponse {
    version: i8,
    data: Vec<SyncManifest>,
}

#[derive(Serialize, Deserialize)]
struct RecordResponse {
    version: i8,
    data: Record,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub id: i32,
    pub sync_hash: String,
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize)]
struct SingleRecordResponse {
    version: i8,
    data: VerboseRecord,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerboseRecord {
    pub id: i32,
    pub name: String,
    pub secret: String,
    pub totp_step: i32,
    pub otp_digits: i32,
    pub algorithm: Option<AccountAlgorithm>,
    pub sync_hash: String,
    pub updated_at: u64,
}

impl VerboseRecord {
    pub fn to_record(&self) -> Record {
        Record {
            id: self.id,
            sync_hash: self.sync_hash.clone(),
            updated_at: self.updated_at,
        }
    }
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

pub async fn get_jwt_token(
    base_url: &str,
    username: &str,
    password: &str,
) -> Result<String, ResponseError> {
    let url = format!("{}/api/login_check", base_url);
    let body = json!({
        "username": username,
        "password": password,
    });

    let response = match make_post(url, body, None).await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let token_response =
        match serde_json::from_str::<TokenResponse>(&response.text().await.unwrap()) {
            Ok(tr) => tr,
            Err(_) => return Err(handle_invalid_response_body()),
        };

    Ok(token_response.token)
}

pub async fn get_manifest(account: &SyncAccount) -> Result<Vec<SyncManifest>, ResponseError> {
    let url = format!("{}/api/records/manifest", account.url);
    let token = account.token.clone();

    let response = match make_get(url, token).await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let manifest_response =
        match serde_json::from_str::<ManifestResponse>(&response.text().await.unwrap()) {
            Ok(m) => m,
            Err(_) => return Err(handle_invalid_response_body()),
        };

    Ok(manifest_response.data)
}

pub async fn authenticate_account(account: SyncAccount) -> Result<SyncAccount, ResponseError> {
    if account.token.is_some() {
        return Ok(account);
    }

    let token = get_jwt_token(&account.url, &account.username, &account.password).await;

    if token.is_ok() {
        return Ok(SyncAccount {
            id: account.id,
            username: account.username,
            password: account.password,
            url: account.url,
            token: Option::from(token.unwrap()),
        });
    }

    Err(token.err().unwrap())
}

pub async fn get_record(
    account: &Account,
    sync_account: &SyncAccount,
) -> Result<Record, ResponseError> {
    let url = format!("{}/api/records", sync_account.url);
    let token = sync_account.token.clone();
    let otp_digits = account.otp_digits;
    let totp_step = account.totp_step;
    let totp_algorithm = account.algorithm.clone();

    let body = json!({
        "name": account.name,
        "secret": encryption::decrypt(&account.secret),
        "otpDigits": otp_digits,
        "totpStep": totp_step,
        "totpAlgorithm": totp_algorithm,
    });

    let response = match make_post(url, body, token).await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let record_response =
        match serde_json::from_str::<RecordResponse>(&response.text().await.unwrap()) {
            Ok(r) => r,
            Err(_) => return Err(handle_invalid_response_body()),
        };

    Ok(record_response.data)
}

pub async fn update_record(
    account: &Account,
    sync_account: &SyncAccount,
) -> Result<Record, ResponseError> {
    let external_id = match account.external_id {
        Some(id) => id,
        None => {
            return Err(ResponseError {
                status: "400".to_string(),
                message: "Missing External Id".to_string(),
            })
        }
    };

    let url = format!("{}/api/records/{}", sync_account.url, external_id);
    let token = sync_account.token.clone();
    let otp_digits = account.otp_digits;
    let totp_step = account.totp_step;
    let totp_algorithm = account.algorithm.clone();

    let body = json!({
        "name": account.name,
        "secret": encryption::decrypt(&account.secret),
        "otpDigits": otp_digits,
        "totpStep": totp_step,
        "totpAlgorithm": totp_algorithm,
    });

    let response = match make_put(url, body, token).await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let record_response =
        match serde_json::from_str::<RecordResponse>(&response.text().await.unwrap()) {
            Ok(r) => r,
            Err(_) => return Err(handle_invalid_response_body()),
        };

    Ok(record_response.data)
}

pub async fn get_single_record(
    id: &i32,
    sync_account: &SyncAccount,
) -> Result<VerboseRecord, ResponseError> {
    let url = format!("{}/api/records/{}", sync_account.url, id);
    let token = sync_account.token.clone();

    let response = match make_get(url, token).await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let record_response =
        match serde_json::from_str::<SingleRecordResponse>(&response.text().await.unwrap()) {
            Ok(r) => r,
            Err(_) => return Err(handle_invalid_response_body()),
        };

    Ok(record_response.data)
}

pub async fn remove_record(id: &i32, sync_account: &SyncAccount) -> Result<bool, ResponseError> {
    let url = format!("{}/api/records/{}", sync_account.url, id);
    let token = sync_account.token.clone();

    match make_delete(url, token).await {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}

async fn make_get(url: String, token: Option<String>) -> Result<Response, ResponseError> {
    let builder = reqwest::Client::builder();
    let mut request_builder = builder
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get(url);

    if token.is_some() {
        request_builder =
            request_builder.header(AUTHORIZATION, "Bearer ".to_owned() + &token.unwrap());
    }

    let res = request_builder.send().await;

    match handle_response(res) {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}

async fn make_post(
    url: String,
    body: Value,
    token: Option<String>,
) -> Result<Response, ResponseError> {
    let builder = reqwest::Client::builder();
    let mut request_builder = builder
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .post(url)
        .json(&body);

    if token.is_some() {
        request_builder =
            request_builder.header(AUTHORIZATION, "Bearer ".to_owned() + &token.unwrap());
    }

    let res = request_builder.send().await;

    match handle_response(res) {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}

async fn make_put(
    url: String,
    body: Value,
    token: Option<String>,
) -> Result<Response, ResponseError> {
    let builder = reqwest::Client::builder();
    let mut request_builder = builder
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .put(url)
        .json(&body);

    if token.is_some() {
        request_builder =
            request_builder.header(AUTHORIZATION, "Bearer ".to_owned() + &token.unwrap());
    }

    let res = request_builder.send().await;

    match handle_response(res) {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}

async fn make_delete(url: String, token: Option<String>) -> Result<Response, ResponseError> {
    let builder = reqwest::Client::builder();
    let mut request_builder = builder
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .delete(url);

    if token.is_some() {
        request_builder =
            request_builder.header(AUTHORIZATION, "Bearer ".to_owned() + &token.unwrap());
    }

    let res = request_builder.send().await;

    match handle_response(res) {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}

fn handle_response(response: Result<Response, Error>) -> Result<Response, ResponseError> {
    let valid_response = match response {
        Ok(res) => res,
        Err(e) => return Err(handle_reqwest_error(e)),
    };

    if !valid_response.status().is_success() {
        let error = ResponseError {
            status: valid_response.status().to_string(),
            message: "Error from server".to_string(),
        };

        return Err(error);
    }

    Ok(valid_response)
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

    ResponseError { status, message }
}

fn handle_invalid_response_body() -> ResponseError {
    ResponseError {
        status: "418".to_string(),
        message: "Could not parse Server response".to_string(),
    }
}
