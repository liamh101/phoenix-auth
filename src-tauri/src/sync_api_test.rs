use crate::database::{Account, AccountAlgorithm, SyncAccount};
use crate::encryption::encrypt;
use crate::sync_api::{
    authenticate_account, get_jwt_token, get_manifest, get_record, make_delete, make_get,
    make_post, make_put, remove_record, update_record,
};
use httpmock::prelude::*;
use serde_json::{json, Value};

#[tokio::test]
async fn test_get_request_no_auth() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(GET).path("/endpoint");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "name": "test" }));
        })
        .await;

    server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/endpoint")
                .header_exists("Authorization");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "name": "fail" }));
        })
        .await;

    let response = make_get(server.url("/endpoint").to_string(), None).await;

    assert_eq!(true, response.is_ok());

    let body = response.unwrap();

    let user: Value =
        serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

    assert_eq!(user.as_object().unwrap().get("name").unwrap(), "test");
}

#[tokio::test]
async fn test_get_request_with_auth() {
    let server = MockServer::start_async().await;

    // Create a mock on the server.
    server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/endpoint")
                .header("Authorization", "Bearer 123456789");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "name": "test" }));
        })
        .await;

    let response = make_get(
        server.url("/endpoint").to_string(),
        Some("123456789".to_string()),
    )
    .await;

    assert_eq!(true, response.is_ok());

    let body = response.unwrap();

    let user: Value =
        serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

    assert_eq!(user.as_object().unwrap().get("name").unwrap(), "test");
}

#[tokio::test]
async fn test_post_request_no_auth() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/endpoint")
                .json_body(json!({ "name": "test" }));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "id": 1, "name": "test" }));
        })
        .await;

    server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/endpoint")
                .header_exists("Authorization")
                .json_body(json!({ "name": "test" }));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "name": "fail" }));
        })
        .await;

    let response = make_post(
        server.url("/endpoint").to_string(),
        json!({ "name": "test" }),
        None,
    )
    .await;

    assert_eq!(true, response.is_ok());

    let body = response.unwrap();

    let user: Value =
        serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

    assert_eq!(user.as_object().unwrap().get("name").unwrap(), "test");
    assert_eq!(user.as_object().unwrap().get("id").unwrap(), 1);
}

#[tokio::test]
async fn test_post_request_with_auth() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/endpoint")
                .json_body(json!({ "name": "test" }))
                .header("Authorization", "Bearer 123456789");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "id": 1, "name": "test" }));
        })
        .await;

    let response = make_post(
        server.url("/endpoint").to_string(),
        json!({ "name": "test" }),
        Some("123456789".to_string()),
    )
    .await;

    assert_eq!(true, response.is_ok());

    let body = response.unwrap();

    let user: Value =
        serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

    assert_eq!(user.as_object().unwrap().get("name").unwrap(), "test");
    assert_eq!(user.as_object().unwrap().get("id").unwrap(), 1);
}

#[tokio::test]
async fn test_put_request_no_auth() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(PUT)
                .path("/endpoint/1")
                .json_body(json!({ "id" : 1, "name": "updated" }));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "id": 1, "name": "updated" }));
        })
        .await;

    server
        .mock_async(|when, then| {
            when.method(PUT)
                .path("/endpoint/1")
                .header_exists("Authorization")
                .json_body(json!({ "id" : 1, "name": "updated" }));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "name": "fail" }));
        })
        .await;

    let response = make_put(
        server.url("/endpoint/1").to_string(),
        json!({ "id": 1, "name": "updated" }),
        None,
    )
    .await;

    assert_eq!(true, response.is_ok());

    let body = response.unwrap();

    let user: Value =
        serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

    assert_eq!(user.as_object().unwrap().get("name").unwrap(), "updated");
    assert_eq!(user.as_object().unwrap().get("id").unwrap(), 1);
}

#[tokio::test]
async fn test_put_request_with_auth() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(PUT)
                .path("/endpoint/1")
                .json_body(json!({ "id": 1, "name": "updated" }))
                .header("Authorization", "Bearer 123456789");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "id": 1, "name": "updated" }));
        })
        .await;

    let response = make_put(
        server.url("/endpoint/1").to_string(),
        json!({ "id": 1, "name": "updated" }),
        Some("123456789".to_string()),
    )
    .await;

    assert_eq!(true, response.is_ok());

    let body = response.unwrap();

    let user: Value =
        serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

    assert_eq!(user.as_object().unwrap().get("name").unwrap(), "updated");
    assert_eq!(user.as_object().unwrap().get("id").unwrap(), 1);
}

#[tokio::test]
async fn test_delete_request_no_auth() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(DELETE).path("/endpoint/1");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "id": 1, "name": "test" }));
        })
        .await;

    server
        .mock_async(|when, then| {
            when.method(DELETE)
                .path("/endpoint/1")
                .header_exists("Authorization");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "name": "fail" }));
        })
        .await;

    let response = make_delete(server.url("/endpoint/1").to_string(), None).await;

    assert_eq!(true, response.is_ok());

    let body = response.unwrap();

    let user: Value =
        serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

    assert_eq!(user.as_object().unwrap().get("name").unwrap(), "test");
    assert_eq!(user.as_object().unwrap().get("id").unwrap(), 1);
}

#[tokio::test]
async fn test_delete_request_with_auth() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(DELETE)
                .path("/endpoint/1")
                .header("Authorization", "Bearer 123456789");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "id": 1, "name": "test" }));
        })
        .await;

    let response = make_delete(
        server.url("/endpoint/1").to_string(),
        Some("123456789".to_string()),
    )
    .await;

    assert_eq!(true, response.is_ok());

    let body = response.unwrap();

    let user: Value =
        serde_json::from_str(&body.text().await.unwrap()).expect("cannot deserialize JSON");

    assert_eq!(user.as_object().unwrap().get("name").unwrap(), "test");
    assert_eq!(user.as_object().unwrap().get("id").unwrap(), 1);
}

#[tokio::test]
async fn test_successfully_validate_account() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/api/login_check")
                .json_body(json!({"username": "test@test.com", "password": "Passw!rd1234"}));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "token": "token1234" }));
        })
        .await;

    let response = get_jwt_token(&server.url(""), &"test@test.com", &"Passw!rd1234").await;

    assert_eq!(true, response.is_ok());

    let body = response.unwrap();

    assert_eq!("token1234", body);
}

#[tokio::test]
async fn test_invalid_validate_account() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/api/login_check")
                .json_body(json!({"username": "test@test.com", "password": "Passw!rd1234"}));
            then.status(401)
                .header("content-type", "application/json")
                .json_body(json!({ "code": 401, "message": "Invalid credentials." }));
        })
        .await;

    let response = get_jwt_token(&server.url(""), &"test@test.com", &"Passw!rd1234").await;

    assert_eq!(true, response.is_err());

    let body = response.err();

    assert_eq!(
        "Error 401 Unauthorized Error from server",
        body.unwrap().formatted_message()
    );
}

#[tokio::test]
async fn test_successful_validate_account_invalid_response() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/api/login_check")
                .json_body(json!({"username": "test@test.com", "password": "Passw!rd1234"}));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "new_token": "token1234" }));
        })
        .await;

    let response = get_jwt_token(&server.url(""), &"test@test.com", &"Passw!rd1234").await;

    assert_eq!(true, response.is_err());

    let body = response.err();

    assert_eq!(
        "Error 418 Could not parse Server response",
        body.unwrap().formatted_message()
    );
}

#[tokio::test]
async fn test_successfully_get_manifest() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
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
        })
        .await;

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
    assert_eq!(1722803353, body[0].updated_at);

    assert_eq!(7, body[1].id);
    assert_eq!(1722803934, body[1].updated_at);
}

#[tokio::test]
async fn test_invalid_get_manifest() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/api/records/manifest")
                .header("Authorization", "Bearer 123456789");
            then.status(401)
                .header("content-type", "application/json")
                .json_body(json!({ "code": 401, "message": "Invalid credentials." }));
        })
        .await;

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

    assert_eq!(
        "Error 401 Unauthorized Error from server",
        body.unwrap().formatted_message()
    );
}

#[tokio::test]
async fn test_successful_get_manifest_invalid_response() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
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
        })
        .await;

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

    assert_eq!(
        "Error 418 Could not parse Server response",
        body.unwrap().formatted_message()
    );
}

#[tokio::test]
async fn test_successfully_authenticate_account() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/api/login_check")
                .json_body(json!({"username": "test@test.com", "password": "Passw!rd1234"}));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "token": "token1234" }));
        })
        .await;

    let account = SyncAccount {
        id: 1,
        username: "test@test.com".to_string(),
        password: "Passw!rd1234".to_string(),
        url: server.url(""),
        token: None,
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

    server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/api/login_check")
                .json_body(json!({"username": "test@test.com", "password": "Passw!rd1234"}));
            then.status(401)
                .header("content-type", "application/json")
                .json_body(json!({ "code": 401, "message": "Invalid credentials." }));
        })
        .await;

    let account = SyncAccount {
        id: 1,
        username: "test@test.com".to_string(),
        password: "Passw!rd1234".to_string(),
        url: server.url(""),
        token: None,
    };

    let response = authenticate_account(account).await;

    assert_eq!(true, response.is_err());

    let body = response.err();

    assert_eq!(
        "Error 401 Unauthorized Error from server",
        body.unwrap().formatted_message()
    );
}

#[tokio::test]
async fn test_successful_authenticate_account_invalid_response() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/api/login_check")
                .json_body(json!({"username": "test@test.com", "password": "Passw!rd1234"}));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "new_token": "token1234" }));
        })
        .await;

    let account = SyncAccount {
        id: 1,
        username: "test@test.com".to_string(),
        password: "Passw!rd1234".to_string(),
        url: server.url(""),
        token: None,
    };

    let response = authenticate_account(account).await;
    assert_eq!(true, response.is_err());

    let body = response.err();

    assert_eq!(
        "Error 418 Could not parse Server response",
        body.unwrap().formatted_message()
    );
}

#[tokio::test]
async fn test_successful_get_record_full() {
    let server = MockServer::start_async().await;
    let secret = "Test123".to_string();

    server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/api/records")
                .json_body(json!({
                    "name": "Full Test Item".to_string(),
                    "secret": secret,
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
        })
        .await;

    let account = Account {
        id: 1,
        name: "Full Test Item".to_string(),
        secret: encrypt(&secret),
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
    assert_eq!("HASHED1234".to_string(), body.sync_hash);
    assert_eq!(1722803353, body.updated_at);
}

#[tokio::test]
async fn test_successful_get_record_required() {
    let server = MockServer::start_async().await;
    let secret = "Test123".to_string();

    server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/api/records")
                .json_body(json!({
                    "name": "Full Test Item".to_string(),
                    "secret": secret,
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
        })
        .await;

    let account = Account {
        id: 1,
        name: "Full Test Item".to_string(),
        secret: encrypt(&secret),
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
    assert_eq!("HASHED1234".to_string(), body.sync_hash);
    assert_eq!(1722803353, body.updated_at);
}

#[tokio::test]
async fn test_invalid_get_record() {
    let server = MockServer::start_async().await;
    let secret = "Test123".to_string();

    server
        .mock_async(|when, then| {
            when.method(POST).path("/api/records").json_body(json!({
                "name": "Full Test Item".to_string(),
                "secret": secret,
                "totpStep": 30,
                "otpDigits": 6,
                "totpAlgorithm": null,
            }));
            then.status(401)
                .header("content-type", "application/json")
                .json_body(json!({ "code": 401, "message": "Invalid credentials." }));
        })
        .await;

    let account = Account {
        id: 1,
        name: "Full Test Item".to_string(),
        secret: encrypt(&secret),
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

    assert_eq!(
        "Error 401 Unauthorized Error from server",
        body.unwrap().formatted_message()
    );
}

#[tokio::test]
async fn test_successful_get_record_invalid_response() {
    let server = MockServer::start_async().await;
    let secret = "Test123".to_string();

    server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/api/records")
                .json_body(json!({
                    "name": "Full Test Item".to_string(),
                    "secret": secret,
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
        })
        .await;

    let account = Account {
        id: 1,
        name: "Full Test Item".to_string(),
        secret: encrypt(&secret),
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

    assert_eq!(
        "Error 418 Could not parse Server response",
        body.unwrap().formatted_message()
    );
}

#[tokio::test]
async fn test_successful_update_record_full() {
    let server = MockServer::start_async().await;
    let secret = "Test123".to_string();

    server
        .mock_async(|when, then| {
            when.method(PUT)
                .path("/api/records/4")
                .json_body(json!({
                    "name": "Full Test Item".to_string(),
                    "secret": secret,
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
                        "id": 4,
                        "syncHash": "HASHED1234".to_string(),
                        "updatedAt": 1722803353,
                    }
                }));
        })
        .await;

    let account = Account {
        id: 1,
        name: "Full Test Item".to_string(),
        secret: encrypt(&secret),
        totp_step: 30,
        otp_digits: 6,
        algorithm: Some(AccountAlgorithm::SHA256),
        external_id: Some(4),
        external_last_updated: Some(1234689),
        external_hash: Some("helpodsa1".to_string()),
        deleted_at: None,
    };

    let sync_account = SyncAccount {
        id: 1,
        username: "test@test.com".to_string(),
        password: "Passw!rd1234".to_string(),
        url: server.url(""),
        token: Some("123456789".to_string()),
    };

    let response = update_record(&account, &sync_account).await;

    assert_eq!(true, response.is_ok());

    let body = response.unwrap();

    assert_eq!(4, body.id);
    assert_eq!("HASHED1234".to_string(), body.sync_hash);
    assert_eq!(1722803353, body.updated_at);
}

#[tokio::test]
async fn test_successful_update_record_required() {
    let server = MockServer::start_async().await;
    let secret = "Test123".to_string();

    server
        .mock_async(|when, then| {
            when.method(PUT)
                .path("/api/records/4")
                .json_body(json!({
                    "name": "Full Test Item".to_string(),
                    "secret": secret,
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
        })
        .await;

    let account = Account {
        id: 1,
        name: "Full Test Item".to_string(),
        secret: encrypt(&secret),
        totp_step: 30,
        otp_digits: 6,
        algorithm: None,
        external_id: Some(4),
        external_last_updated: Some(1234),
        external_hash: Some("Helosfaf".to_string()),
        deleted_at: None,
    };

    let sync_account = SyncAccount {
        id: 1,
        username: "test@test.com".to_string(),
        password: "Passw!rd1234".to_string(),
        url: server.url(""),
        token: Some("123456789".to_string()),
    };

    let response = update_record(&account, &sync_account).await;

    assert_eq!(true, response.is_ok());

    let body = response.unwrap();

    assert_eq!(12, body.id);
    assert_eq!("HASHED1234".to_string(), body.sync_hash);
    assert_eq!(1722803353, body.updated_at);
}

#[tokio::test]
async fn test_invalid_update_record() {
    let server = MockServer::start_async().await;
    let secret = "Test123".to_string();

    server
        .mock_async(|when, then| {
            when.method(PUT).path("/api/records/2").json_body(json!({
                "name": "Full Test Item".to_string(),
                "secret": secret,
                "totpStep": 30,
                "otpDigits": 6,
                "totpAlgorithm": null,
            }));
            then.status(401)
                .header("content-type", "application/json")
                .json_body(json!({ "code": 401, "message": "Invalid credentials." }));
        })
        .await;

    let account = Account {
        id: 1,
        name: "Full Test Item".to_string(),
        secret: encrypt(&secret),
        totp_step: 30,
        otp_digits: 6,
        algorithm: None,
        external_id: Some(2),
        external_last_updated: Some(1243),
        external_hash: Some("Hello".to_string()),
        deleted_at: None,
    };

    let sync_account = SyncAccount {
        id: 1,
        username: "test@test.com".to_string(),
        password: "Passw!rd1234".to_string(),
        url: server.url(""),
        token: None,
    };

    let response = update_record(&account, &sync_account).await;

    assert_eq!(true, response.is_err());

    let body = response.err();

    assert_eq!(
        "Error 401 Unauthorized Error from server",
        body.unwrap().formatted_message()
    );
}

#[tokio::test]
async fn test_invalid_update_record_missing_external() {
    let secret = "Test123".to_string();

    let account = Account {
        id: 1,
        name: "Full Test Item".to_string(),
        secret: encrypt(&secret),
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
        url: "".to_string(),
        token: None,
    };

    let response = update_record(&account, &sync_account).await;

    assert_eq!(true, response.is_err());

    let body = response.err();

    assert_eq!(
        "Error 400 Missing External Id",
        body.unwrap().formatted_message()
    );
}

#[tokio::test]
async fn test_successful_update_record_invalid_response() {
    let server = MockServer::start_async().await;
    let secret = "Test123".to_string();

    server
        .mock_async(|when, then| {
            when.method(PUT)
                .path("/api/records/12")
                .json_body(json!({
                    "name": "Full Test Item".to_string(),
                    "secret": secret,
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
        })
        .await;

    let account = Account {
        id: 1,
        name: "Full Test Item".to_string(),
        secret: encrypt(&secret),
        totp_step: 30,
        otp_digits: 6,
        algorithm: None,
        external_id: Some(12),
        external_last_updated: Some(1235),
        external_hash: Some("Heelo".to_string()),
        deleted_at: None,
    };

    let sync_account = SyncAccount {
        id: 1,
        username: "test@test.com".to_string(),
        password: "Passw!rd1234".to_string(),
        url: server.url(""),
        token: Some("123456789".to_string()),
    };

    let response = update_record(&account, &sync_account).await;
    assert_eq!(true, response.is_err());

    let body = response.err();

    assert_eq!(
        "Error 418 Could not parse Server response",
        body.unwrap().formatted_message()
    );
}

#[tokio::test]
async fn test_successful_delete_record() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(DELETE)
                .path("/api/records/8")
                .header("Authorization", "Bearer 123456789");
            then.status(201);
        })
        .await;

    let sync_account = SyncAccount {
        id: 1,
        username: "test@test.com".to_string(),
        password: "Passw!rd1234".to_string(),
        url: server.url(""),
        token: Some("123456789".to_string()),
    };

    let response = remove_record(&8, &sync_account).await;

    assert_eq!(true, response.is_ok());
    assert_eq!(true, response.unwrap());
}

#[tokio::test]
async fn test_delete_record_missing_record() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(DELETE)
                .path("/api/records/8")
                .header("Authorization", "Bearer 123456789");
            then.status(404);
        })
        .await;

    let sync_account = SyncAccount {
        id: 1,
        username: "test@test.com".to_string(),
        password: "Passw!rd1234".to_string(),
        url: server.url(""),
        token: Some("123456789".to_string()),
    };

    let response = remove_record(&8, &sync_account).await;

    assert_eq!(true, response.is_err());
    assert_eq!(
        "Error 404 Not Found Error from server",
        response.err().unwrap().formatted_message()
    );
}

#[tokio::test]
async fn test_delete_record_invalid() {
    let server = MockServer::start_async().await;

    server
        .mock_async(|when, then| {
            when.method(DELETE).path("/api/records/8");
            then.status(401)
                .json_body(json!({ "code": 401, "message": "Invalid credentials." }));
        })
        .await;

    let sync_account = SyncAccount {
        id: 1,
        username: "test@test.com".to_string(),
        password: "Passw!rd1234".to_string(),
        url: server.url(""),
        token: None,
    };

    let response = remove_record(&8, &sync_account).await;

    assert_eq!(true, response.is_err());
    assert_eq!(
        "Error 401 Unauthorized Error from server",
        response.err().unwrap().formatted_message()
    );
}
