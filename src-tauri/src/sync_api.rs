use reqwest::{Error, Response};
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::database::{Account, AccountAlgorithm, SyncAccount};
use crate::encryption;

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct SyncManifest {
    pub id: i32,
    pub updatedAt: u64,
}

#[derive(Serialize, Deserialize)]
struct ManifestResponse {
    version: i8,
    data: Vec<SyncManifest>
}

#[derive(Serialize, Deserialize)]
struct RecordResponse {
    version: i8,
    data: Record,
}

#[derive(Serialize, Deserialize)]
pub struct Record {
    pub id: i32,
    pub syncHash: String,
    pub updatedAt: u64,
}

#[derive(Serialize, Deserialize)]
struct SingleRecordResponse {
    version: i8,
    data: VerboseRecord,
}

#[derive(Serialize, Deserialize)]
pub struct VerboseRecord {
    pub id: i32,
    pub name: String,
    pub secret: String,
    pub totpStep: i32,
    pub otpDigits: i32,
    pub algorithm: Option<AccountAlgorithm>,
    pub syncHash: String,
    pub updatedAt: u64,
}

impl VerboseRecord {
    pub fn to_record(&self) -> Record {
        Record {
            id: self.id,
            syncHash: self.syncHash.clone(),
            updatedAt: self.updatedAt,
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

pub async fn get_jwt_token(base_url: &str, username: &str, password: &str) -> Result<String, ResponseError> {
    let url = format!("{}/api/login_check", base_url);
    let body = json!({
        "username": username,
        "password": password,
    });

    let response = match make_post(url, body, None).await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let token_response: TokenResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    Ok(token_response.token)
}

pub async fn get_manifest(account: &SyncAccount) -> Result<Vec<SyncManifest>, ResponseError> {
    let url = format!("{}/api/records/manifest", account.url);
    let token = account.token.clone();

    let response = match make_get(url, token).await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let manifest_response: ManifestResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    Ok(manifest_response.data)
}

pub async fn authenticate_account(account: SyncAccount) -> Result<SyncAccount, ResponseError> {
    if account.token.is_some() {
        return Ok(account)
    }

    let token = get_jwt_token(&account.url, &account.username, &account.password).await;

    if token.is_ok() {
        return Ok(SyncAccount {
            id:account.id,
            username: account.username,
            password: account.password,
            url: account.url,
            token: Option::from(token.unwrap()),
        })
    }

    return Err(token.err().unwrap())
}

pub async fn get_record(account: &Account, sync_account: &SyncAccount) -> Result<Record, ResponseError> {
    let url = format!("{}/api/records", sync_account.url);
    let token = sync_account.token.clone();
    let otp_digits = account.otp_digits;
    let totp_step = account.totp_step;
    let totp_algorithm = account.algorithm.clone();

    let body = json!({
        "name": account.name,
        "secret": account.secret,
        "otpDigits": otp_digits,
        "totpStep": totp_step,
        "totpAlgorithm": totp_algorithm,
    });

    let response = match make_post(url, body, token).await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let record_response: RecordResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    Ok(record_response.data)
}

pub async fn update_record(account: &Account, sync_account: &SyncAccount) -> Result<Record, ResponseError> {
    let url = format!("{}/api/records/{}", sync_account.url, account.external_id.unwrap());
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

    let record_response: RecordResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    Ok(record_response.data)
}

pub async fn get_single_record(id: &i32, sync_account: &SyncAccount) -> Result<VerboseRecord, ResponseError> {
    let url = format!("{}/api/records/{}", sync_account.url, id);
    let token = sync_account.token.clone();

    let response = match make_get(url, token).await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let record_response: SingleRecordResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    Ok(record_response.data)
}

async fn make_get(url: String, token: Option<String>) -> Result<Response, ResponseError> {
    let builder = reqwest::Client::builder();
    let mut request_builder = builder
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get(url);

    if token.is_some() {
        request_builder = request_builder.header(AUTHORIZATION, "Bearer ".to_owned() + &token.unwrap());
    }

    let res =
        request_builder
        .send()
        .await;

    return match handle_response(res) {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    };
}

async fn make_post(url: String, body: Value, token: Option<String>) -> Result<Response, ResponseError> {
    let builder = reqwest::Client::builder();
    let mut request_builder = builder
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .post(url)
        .json(&body);


    if token.is_some() {
        request_builder = request_builder.header(AUTHORIZATION, "Bearer ".to_owned() + &token.unwrap());
    }

    let res =
        request_builder
            .send()
            .await;

    return match handle_response(res) {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    };
}

async fn make_put(url: String, body: Value, token: Option<String>) -> Result<Response, ResponseError> {
    let builder = reqwest::Client::builder();
    let mut request_builder = builder
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .put(url)
        .json(&body);


    if token.is_some() {
        request_builder = request_builder.header(AUTHORIZATION, "Bearer ".to_owned() + &token.unwrap());
    }

    let res =
        request_builder
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