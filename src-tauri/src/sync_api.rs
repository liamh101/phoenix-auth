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
    data: Vec<SyncManifest>,
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

    let token_response = match serde_json::from_str::<TokenResponse>(&response.text().await.unwrap()) {
        Ok(tr) => tr,
        Err(_) => return Err(handle_invalid_response_body())
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

    let manifest_response = match serde_json::from_str::<ManifestResponse>(&response.text().await.unwrap()) {
        Ok(m) => m,
        Err(_) => return Err(handle_invalid_response_body())
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

    return Err(token.err().unwrap());
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

    let record_response = match serde_json::from_str::<RecordResponse>(&response.text().await.unwrap()) {
        Ok(r) => r,
        Err(_) => return Err(handle_invalid_response_body())
    };

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

async fn make_delete(url: String, token: Option<String>) -> Result<Response, ResponseError> {
    let builder = reqwest::Client::builder();
    let mut request_builder = builder
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .delete(url);

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
            message: "Error from server".to_string(),
        };

        return Err(error);
    }

    return Ok(valid_response);
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
        message,
    };
}

fn handle_invalid_response_body() -> ResponseError {
    ResponseError { status: "418".to_string(), message: "Could not parse Server response".to_string() }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use serde_json::{json, Value};
    use crate::database::{Account, AccountAlgorithm, SyncAccount};
    use crate::sync_api::{authenticate_account, get_jwt_token, get_manifest, get_record, make_delete, make_get, make_post, make_put};

    #[tokio::test]
    async fn test_get_request_no_auth() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(GET)
                .path("/endpoint");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "name": "test" }));
        }).await;

        let fail_mock = server.mock_async(|when, then| {
            when.method(GET)
                .path("/endpoint")
                .header_exists("Authorization");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "name": "fail" }));
        }).await;

        let response = make_get(server.url("/endpoint").to_string(), None).await;

        assert_eq!(true, response.is_ok());

        let body = response.unwrap();

        let user: Value = serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

        assert_eq!(user.as_object().unwrap().get("name").unwrap(), "test");
    }

    #[tokio::test]
    async fn test_get_request_with_auth() {
        let server = MockServer::start_async().await;

        // Create a mock on the server.
        let hello_mock = server.mock_async(|when, then| {
            when.method(GET)
                .path("/endpoint")
                .header("Authorization", "Bearer 123456789");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "name": "test" }));
        }).await;

        let response = make_get(server.url("/endpoint").to_string(), Some("123456789".to_string())).await;

        assert_eq!(true, response.is_ok());

        let body = response.unwrap();

        let user: Value = serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

        assert_eq!(user.as_object().unwrap().get("name").unwrap(), "test");
    }

    #[tokio::test]
    async fn test_post_request_no_auth() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(POST)
                .path("/endpoint")
                .json_body(json!({ "name": "test" }));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "id": 1, "name": "test" }));
        }).await;

        let fail_mock = server.mock_async(|when, then| {
            when.method(POST)
                .path("/endpoint")
                .header_exists("Authorization")
                .json_body(json!({ "name": "test" }));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "name": "fail" }));
        }).await;

        let response = make_post(server.url("/endpoint").to_string(), json!({ "name": "test" }), None).await;

        assert_eq!(true, response.is_ok());

        let body = response.unwrap();

        let user: Value = serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

        assert_eq!(user.as_object().unwrap().get("name").unwrap(), "test");
        assert_eq!(user.as_object().unwrap().get("id").unwrap(), 1);
    }

    #[tokio::test]
    async fn test_post_request_with_auth() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(POST)
                .path("/endpoint")
                .json_body(json!({ "name": "test" }))
                .header("Authorization", "Bearer 123456789");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "id": 1, "name": "test" }));
        }).await;

        let response = make_post(server.url("/endpoint").to_string(), json!({ "name": "test" }), Some("123456789".to_string())).await;

        assert_eq!(true, response.is_ok());

        let body = response.unwrap();

        let user: Value = serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

        assert_eq!(user.as_object().unwrap().get("name").unwrap(), "test");
        assert_eq!(user.as_object().unwrap().get("id").unwrap(), 1);
    }

    #[tokio::test]
    async fn test_put_request_no_auth() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(PUT)
                .path("/endpoint/1")
                .json_body(json!({ "id" : 1, "name": "updated" }));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "id": 1, "name": "updated" }));
        }).await;

        let fail_mock = server.mock_async(|when, then| {
            when.method(PUT)
                .path("/endpoint/1")
                .header_exists("Authorization")
                .json_body(json!({ "id" : 1, "name": "updated" }));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "name": "fail" }));
        }).await;

        let response = make_put(server.url("/endpoint/1").to_string(), json!({ "id": 1, "name": "updated" }), None).await;

        assert_eq!(true, response.is_ok());

        let body = response.unwrap();

        let user: Value = serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

        assert_eq!(user.as_object().unwrap().get("name").unwrap(), "updated");
        assert_eq!(user.as_object().unwrap().get("id").unwrap(), 1);
    }

    #[tokio::test]
    async fn test_put_request_with_auth() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(PUT)
                .path("/endpoint/1")
                .json_body(json!({ "id": 1, "name": "updated" }))
                .header("Authorization", "Bearer 123456789");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "id": 1, "name": "updated" }));
        }).await;

        let response = make_put(server.url("/endpoint/1").to_string(), json!({ "id": 1, "name": "updated" }), Some("123456789".to_string())).await;

        assert_eq!(true, response.is_ok());

        let body = response.unwrap();

        let user: Value = serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

        assert_eq!(user.as_object().unwrap().get("name").unwrap(), "updated");
        assert_eq!(user.as_object().unwrap().get("id").unwrap(), 1);
    }

    #[tokio::test]
    async fn test_delete_request_no_auth() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(DELETE)
                .path("/endpoint/1");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "id": 1, "name": "test" }));
        }).await;

        let fail_mock = server.mock_async(|when, then| {
            when.method(DELETE)
                .path("/endpoint/1")
                .header_exists("Authorization");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "name": "fail" }));
        }).await;

        let response = make_delete(server.url("/endpoint/1").to_string(), None).await;

        assert_eq!(true, response.is_ok());

        let body = response.unwrap();

        let user: Value = serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

        assert_eq!(user.as_object().unwrap().get("name").unwrap(), "test");
        assert_eq!(user.as_object().unwrap().get("id").unwrap(), 1);
    }

    #[tokio::test]
    async fn test_delete_request_with_auth() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(DELETE)
                .path("/endpoint/1")
                .header("Authorization", "Bearer 123456789");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "id": 1, "name": "test" }));
        }).await;


        let response = make_delete(server.url("/endpoint/1").to_string(), Some("123456789".to_string())).await;

        assert_eq!(true, response.is_ok());

        let body = response.unwrap();

        let user: Value = serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

        assert_eq!(user.as_object().unwrap().get("name").unwrap(), "test");
        assert_eq!(user.as_object().unwrap().get("id").unwrap(), 1);
    }

    #[tokio::test]
    async fn test_successfully_validate_account() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(POST)
                .path("/api/login_check")
                .json_body(json!({"username": "test@test.com", "password": "Passw!rd1234"}));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "token": "token1234" }));
        }).await;

        let response = get_jwt_token(&server.url(""), &"test@test.com", &"Passw!rd1234").await;

        assert_eq!(true, response.is_ok());

        let body = response.unwrap();

        assert_eq!("token1234", body);
    }

    #[tokio::test]
    async fn test_invalid_validate_account() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(POST)
                .path("/api/login_check")
                .json_body(json!({"username": "test@test.com", "password": "Passw!rd1234"}));
            then.status(401)
                .header("content-type", "application/json")
                .json_body(json!({ "code": 401, "message": "Invalid credentials." }));
        }).await;

        let response = get_jwt_token(&server.url(""), &"test@test.com", &"Passw!rd1234").await;

        assert_eq!(true, response.is_err());

        let body = response.err();

        assert_eq!("Error 401 Unauthorized Error from server", body.unwrap().formatted_message());
    }

    #[tokio::test]
    async fn test_successful_validate_account_invalid_response() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(POST)
                .path("/api/login_check")
                .json_body(json!({"username": "test@test.com", "password": "Passw!rd1234"}));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "new_token": "token1234" }));
        }).await;

        let response = get_jwt_token(&server.url(""), &"test@test.com", &"Passw!rd1234").await;

        assert_eq!(true, response.is_err());

        let body = response.err();

        assert_eq!("Error 418 Could not parse Server response", body.unwrap().formatted_message());
    }

    #[tokio::test]
    async fn test_successfully_get_manifest() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(GET)
                .path("/api/records/manifest")
                .header("Authorization", "Bearer 123456789");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                "version": 1,
                "data": [
                    {
                        "id": 6,
                        "updatedAt": 1722803353
                    },
                    {
                        "id": 7,
                        "updatedAt": 1722803934
                    }
                ]
            }));
        }).await;

        let sync_account = SyncAccount {
            id: 1,
            username: "test@test.com".to_string(),
            password: "Password".to_string(),
            url: server.url("").to_string(),
            token: Some("123456789".to_string()),
        };
        let response = get_manifest(&sync_account).await;

        assert_eq!(true, response.is_ok());

        let body = response.unwrap();

        assert_eq!(6, body[0].id);
        assert_eq!(1722803353, body[0].updatedAt);

        assert_eq!(7, body[1].id);
        assert_eq!(1722803934, body[1].updatedAt);
    }

    #[tokio::test]
    async fn test_invalid_get_manifest() {
        let server = MockServer::start_async().await;

        let invalid_mock = server.mock_async(|when, then| {
            when.method(GET)
                .path("/api/records/manifest")
                .header("Authorization", "Bearer 123456789");
            then.status(401)
                .header("content-type", "application/json")
                .json_body(json!({ "code": 401, "message": "Invalid credentials." }));
        }).await;

        let sync_account = SyncAccount {
            id: 1,
            username: "test@test.com".to_string(),
            password: "Password".to_string(),
            url: server.url("").to_string(),
            token: Some("123456789".to_string()),
        };
        let response = get_manifest(&sync_account).await;

        assert_eq!(true, response.is_err());

        let body = response.err();

        assert_eq!("Error 401 Unauthorized Error from server", body.unwrap().formatted_message());
    }

    #[tokio::test]
    async fn test_successful_get_manifest_invalid_response() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(GET)
                .path("/api/records/manifest")
                .header("Authorization", "Bearer 123456789");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                "version": 2,
                "data": [
                    {
                        "id": 6,
                        "updated_at": "2024-09-26"
                    },
                    {
                        "id": 7,
                        "updated_at": "2024-09-26"
                    }
                ]
            }));
        }).await;

        let sync_account = SyncAccount {
            id: 1,
            username: "test@test.com".to_string(),
            password: "Password".to_string(),
            url: server.url("").to_string(),
            token: Some("123456789".to_string()),
        };
        let response = get_manifest(&sync_account).await;

        assert_eq!(true, response.is_err());

        let body = response.err();

        assert_eq!("Error 418 Could not parse Server response", body.unwrap().formatted_message());
    }

    #[tokio::test]
    async fn test_successfully_authenticate_account() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(POST)
                .path("/api/login_check")
                .json_body(json!({"username": "test@test.com", "password": "Passw!rd1234"}));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "token": "token1234" }));
        }).await;

        let account = SyncAccount {
            id: 1,
            username: "test@test.com".to_string(),
            password: "Passw!rd1234".to_string(),
            url: server.url(""),
            token: None
        };

        let response = authenticate_account(account).await;

        assert_eq!(true, response.is_ok());

        let body = response.unwrap();

        assert_eq!(1, body.id);
        assert_eq!("test@test.com".to_string(), body.username);
        assert_eq!("Passw!rd1234".to_string(), body.password);
        assert_eq!(server.url(""), body.url);
        assert_eq!(Some("token1234".to_string()), body.token);
    }

    #[tokio::test]
    async fn test_invalid_authenticate_account() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(POST)
                .path("/api/login_check")
                .json_body(json!({"username": "test@test.com", "password": "Passw!rd1234"}));
            then.status(401)
                .header("content-type", "application/json")
                .json_body(json!({ "code": 401, "message": "Invalid credentials." }));
        }).await;

        let account = SyncAccount {
            id: 1,
            username: "test@test.com".to_string(),
            password: "Passw!rd1234".to_string(),
            url: server.url(""),
            token: None
        };

        let response = authenticate_account(account).await;

        assert_eq!(true, response.is_err());

        let body = response.err();

        assert_eq!("Error 401 Unauthorized Error from server", body.unwrap().formatted_message());
    }

    #[tokio::test]
    async fn test_successful_authenticate_account_invalid_response() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(POST)
                .path("/api/login_check")
                .json_body(json!({"username": "test@test.com", "password": "Passw!rd1234"}));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "new_token": "token1234" }));
        }).await;

        let account = SyncAccount {
            id: 1,
            username: "test@test.com".to_string(),
            password: "Passw!rd1234".to_string(),
            url: server.url(""),
            token: None
        };

        let response = authenticate_account(account).await;
        assert_eq!(true, response.is_err());

        let body = response.err();

        assert_eq!("Error 418 Could not parse Server response", body.unwrap().formatted_message());
    }

    #[tokio::test]
    async fn test_successful_get_record_full() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(POST)
                .path("/api/records")
                .json_body(json!({
                    "name": "Full Test Item".to_string(),
                    "secret": "Test123".to_string(),
                    "totpStep": 30,
                    "otpDigits": 6,
                    "totpAlgorithm": "SHA256",
                }))
                .header("Authorization", "Bearer 123456789");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "version": 1,
                    "data": {
                        "id": 12,
                        "syncHash": "HASHED1234".to_string(),
                        "updatedAt": 1722803353,
                    }
                }));
        }).await;

        let account = Account {
            id: 1,
            name: "Full Test Item".to_string(),
            secret: "Test123".to_string(),
            totp_step: 30,
            otp_digits: 6,
            algorithm: Some(AccountAlgorithm::SHA256),
            external_id: None,
            external_last_updated: None,
            external_hash: None,
            deleted_at: None,
        };

        let sync_account = SyncAccount {
            id: 1,
            username: "test@test.com".to_string(),
            password: "Passw!rd1234".to_string(),
            url: server.url(""),
            token: Some("123456789".to_string()),
        };

        let response = get_record(&account, &sync_account).await;

        assert_eq!(true, response.is_ok());

        let body = response.unwrap();

        assert_eq!(12, body.id);
        assert_eq!("HASHED1234".to_string(), body.syncHash);
        assert_eq!(1722803353, body.updatedAt);
    }

    #[tokio::test]
    async fn test_successful_get_record_required() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(POST)
                .path("/api/records")
                .json_body(json!({
                    "name": "Full Test Item".to_string(),
                    "secret": "Test123".to_string(),
                    "totpStep": 30,
                    "otpDigits": 6,
                    "totpAlgorithm": null,
                }))
                .header("Authorization", "Bearer 123456789");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "version": 1,
                    "data": {
                        "id": 12,
                        "syncHash": "HASHED1234".to_string(),
                        "updatedAt": 1722803353,
                    }
                }));
        }).await;

        let account = Account {
            id: 1,
            name: "Full Test Item".to_string(),
            secret: "Test123".to_string(),
            totp_step: 30,
            otp_digits: 6,
            algorithm: None,
            external_id: None,
            external_last_updated: None,
            external_hash: None,
            deleted_at: None,
        };

        let sync_account = SyncAccount {
            id: 1,
            username: "test@test.com".to_string(),
            password: "Passw!rd1234".to_string(),
            url: server.url(""),
            token: Some("123456789".to_string()),
        };

        let response = get_record(&account, &sync_account).await;

        assert_eq!(true, response.is_ok());

        let body = response.unwrap();

        assert_eq!(12, body.id);
        assert_eq!("HASHED1234".to_string(), body.syncHash);
        assert_eq!(1722803353, body.updatedAt);
    }

    async fn test_invalid_get_record() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(POST)
                .path("/api/records")
                .json_body(json!({
                    "name": "Full Test Item".to_string(),
                    "secret": "Test123".to_string(),
                    "totpStep": 30,
                    "otpDigits": 6,
                    "totpAlgorithm": null,
                }));
            then.status(401)
                .header("content-type", "application/json")
                .json_body(json!({ "code": 401, "message": "Invalid credentials." }));
        }).await;

        let account = Account {
            id: 1,
            name: "Full Test Item".to_string(),
            secret: "Test123".to_string(),
            totp_step: 30,
            otp_digits: 6,
            algorithm: None,
            external_id: None,
            external_last_updated: None,
            external_hash: None,
            deleted_at: None,
        };

        let sync_account = SyncAccount {
            id: 1,
            username: "test@test.com".to_string(),
            password: "Passw!rd1234".to_string(),
            url: server.url(""),
            token: None,
        };

        let response = get_record(&account, &sync_account).await;

        assert_eq!(true, response.is_err());

        let body = response.err();

        assert_eq!("Error 401 Unauthorized Error from server", body.unwrap().formatted_message());
    }

    async fn test_successful_get_record_invalid_response() {
        let server = MockServer::start_async().await;

        let success_mock = server.mock_async(|when, then| {
            when.method(POST)
                .path("/api/records")
                .json_body(json!({
                    "name": "Full Test Item".to_string(),
                    "secret": "Test123".to_string(),
                    "totpStep": 30,
                    "otpDigits": 6,
                    "totpAlgorithm": null,
                }))
                .header("Authorization", "Bearer 123456789");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "version": 2,
                    "data": {
                        "id": 12,
                        "syncHash": "HASHED1234".to_string(),
                        "updatedAt": "2024-06-08 12:00:00",
                    }
                }));
        }).await;

        let account = Account {
            id: 1,
            name: "Full Test Item".to_string(),
            secret: "Test123".to_string(),
            totp_step: 30,
            otp_digits: 6,
            algorithm: None,
            external_id: None,
            external_last_updated: None,
            external_hash: None,
            deleted_at: None,
        };

        let sync_account = SyncAccount {
            id: 1,
            username: "test@test.com".to_string(),
            password: "Passw!rd1234".to_string(),
            url: server.url(""),
            token: Some("123456789".to_string()),
        };

        let response = get_record(&account, &sync_account).await;
        assert_eq!(true, response.is_err());

        let body = response.err();

        assert_eq!("Error 418 Could not parse Server response", body.unwrap().formatted_message());
    }
}