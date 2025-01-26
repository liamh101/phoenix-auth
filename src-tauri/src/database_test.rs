use crate::database::AccountAlgorithm::{SHA1, SHA512};
use crate::database::{
    create_new_account, create_sync_account, create_sync_log, delete_account, delete_sync_account,
    get_account_details_by_id, get_all_accounts, get_main_sync_account, get_soft_deleted_accounts,
    get_sync_logs, initialize_database, set_remote_account, update_existing_account,
    update_sync_account, AccountAlgorithm, SyncAccount, SyncLogType,
};
use crate::sync_api::Record;
use libotp::HOTPAlgorithm;
use rusqlite::Connection;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::AppHandle;
use crate::encryption::legacy_encrypt;

const SQLITE_TEST_NAME: &str = "Phoenix_test.sqlite";

#[test]
fn can_parse_sha1() {
    let _algorithm = AccountAlgorithm::string_to_algorithm("SHA1".to_string()).unwrap();
    let is_valid = matches!(AccountAlgorithm::SHA1, _algorithm);

    assert_eq!(true, is_valid);
}

#[test]
fn can_parse_sha256() {
    let _algorithm = AccountAlgorithm::string_to_algorithm("SHA256".to_string()).unwrap();
    let is_valid = matches!(AccountAlgorithm::SHA256, _algorithm);

    assert_eq!(true, is_valid);
}

#[test]
fn can_parse_sha512() {
    let _algorithm = AccountAlgorithm::string_to_algorithm("SHA512".to_string()).unwrap();
    let is_valid = matches!(AccountAlgorithm::SHA512, _algorithm);

    assert_eq!(true, is_valid);
}

#[test]
fn can_parse_invalid() {
    let is_none = AccountAlgorithm::string_to_algorithm("Hello world".to_string()).is_none();

    assert_eq!(true, is_none);
}

#[test]
fn can_translate_sha1() {
    assert_eq!(
        true,
        matches!(
            AccountAlgorithm::SHA1.to_hotp_algorithm(),
            HOTPAlgorithm::HMACSHA1
        )
    );
}

#[test]
fn can_translate_sha256() {
    assert_eq!(
        true,
        matches!(
            AccountAlgorithm::SHA256.to_hotp_algorithm(),
            HOTPAlgorithm::HMACSHA256
        )
    );
}

#[test]
fn can_translate_sha512() {
    assert_eq!(
        true,
        matches!(
            AccountAlgorithm::SHA512.to_hotp_algorithm(),
            HOTPAlgorithm::HMACSHA512
        )
    );
}

#[test]
fn create_new_account_full() {
    let db = initialize_test_database().unwrap();
    let name = "New Full Test";
    let secret = "HelloWorld";
    let digits = 8;
    let step = 30;
    let algorithm = "SHA1";

    let result = create_new_account(&name, &secret, &digits, &step, &algorithm, &db);

    assert_eq!(true, result.is_ok());

    let account = result.unwrap();
    assert_eq!(true, account.id != 0);
    assert_eq!(name.to_string(), account.name);
    assert_eq!(secret.to_string(), account.secret);
    assert_eq!(digits, account.otp_digits);
    assert_eq!(step, account.totp_step);

    assert_eq!(true, account.algorithm.is_some());
    assert_eq!(SHA1, account.algorithm.unwrap());
}

#[test]
fn create_new_account_required() {
    let db = initialize_test_database().unwrap();
    let name = "Required Test";
    let secret = "HelloWorld2";
    let digits = 8;
    let step = 30;
    let algorithm = "";

    let result = create_new_account(&name, &secret, &digits, &step, &algorithm, &db);

    assert_eq!(true, result.is_ok());

    let account = result.unwrap();
    assert_eq!(true, account.id != 0);
    assert_eq!(name.to_string(), account.name);
    assert_eq!(secret.to_string(), account.secret);
    assert_eq!(digits, account.otp_digits);
    assert_eq!(step, account.totp_step);

    assert_eq!(true, account.algorithm.is_none());
}

#[test]
fn update_existing_account_full() {
    let db = initialize_test_database().unwrap();
    let name = "Full Test";
    let secret = "HelloWorld";
    let digits = 8;
    let step = 30;
    let algorithm = "SHA1";

    let updated_name = "Full Test Update";
    let updated_secret = "HelloWorld245";
    let updated_digits = 12;
    let updated_step = 60;
    let updated_algorithm = "SHA512";

    let original_account =
        create_new_account(&name, &secret, &digits, &step, &algorithm, &db).unwrap();
    let result = update_existing_account(
        &original_account.id,
        &updated_name,
        &updated_secret,
        updated_digits,
        updated_step,
        &updated_algorithm,
        &db,
    );

    assert_eq!(true, result.is_ok());

    let updated_account = result.unwrap();

    assert_eq!(original_account.id, updated_account.id);
    assert_eq!(updated_name.to_string(), updated_account.name);
    assert_eq!(updated_secret.to_string(), updated_account.secret);
    assert_eq!(updated_digits, updated_account.otp_digits);
    assert_eq!(updated_step, updated_account.totp_step);

    assert_eq!(true, updated_account.algorithm.is_some());
    assert_eq!(SHA512, updated_account.algorithm.unwrap());
}

#[test]
fn update_existing_account_partial() {
    let db = initialize_test_database().unwrap();
    let name = "Full Test";
    let secret = "HelloWorld";
    let digits = 8;
    let step = 30;
    let algorithm = "SHA1";

    let updated_algorithm = "";

    let original_account =
        create_new_account(&name, &secret, &digits, &step, &algorithm, &db).unwrap();
    let result = update_existing_account(
        &original_account.id,
        &original_account.name,
        &original_account.secret,
        original_account.otp_digits,
        original_account.totp_step,
        &updated_algorithm,
        &db,
    );

    assert_eq!(true, result.is_ok());

    let updated_account = result.unwrap();

    assert_eq!(original_account.id, updated_account.id);
    assert_eq!(original_account.name.to_string(), updated_account.name);
    assert_eq!(original_account.secret.to_string(), updated_account.secret);
    assert_eq!(original_account.otp_digits, updated_account.otp_digits);
    assert_eq!(original_account.totp_step, updated_account.totp_step);

    assert_eq!(true, updated_account.algorithm.is_none());
}

#[test]
fn get_all_accounts_order() {
    let db = initialize_test_database().unwrap();
    reset_db(&db).expect("Cant reset");

    let expected_second = create_new_account("AB Record", "1234", &8, &30, "", &db).unwrap();
    let expected_third = create_new_account("AC Record", "2134", &4, &15, "", &db).unwrap();
    let expected_first = create_new_account("AA Record", "9284", &12, &60, "", &db).unwrap();

    let result = get_all_accounts(&db, "");

    assert_eq!(true, result.is_ok());

    let accounts = result.unwrap();

    assert_eq!(3, accounts.len());

    assert_eq!(expected_first.id, accounts[0].id);
    assert_eq!(expected_first.name, accounts[0].name);
    assert_eq!("", accounts[0].secret);

    assert_eq!(expected_second.id, accounts[1].id);
    assert_eq!(expected_second.name, accounts[1].name);
    assert_eq!("", accounts[1].secret);

    assert_eq!(expected_third.id, accounts[2].id);
    assert_eq!(expected_third.name, accounts[2].name);
    assert_eq!("", accounts[2].secret);
}

#[test]
fn get_account_details_by_id_default() {
    let db = initialize_test_database().unwrap();
    reset_db(&db).expect("Cant reset");

    let expected = create_new_account("AA Record", "9284", &12, &60, "SHA1", &db).unwrap();

    let result = get_account_details_by_id(expected.id as u32, &db);

    assert_eq!(true, result.is_ok());

    let account = result.unwrap();

    assert_eq!(expected.id, account.id);
    assert_eq!("AA Record", account.name);
    assert_eq!("9284", account.secret);
    assert_eq!(12, account.otp_digits);
    assert_eq!(60, account.totp_step);
    assert_eq!(true, account.algorithm.is_some());
    assert_eq!(SHA1, account.algorithm.unwrap());
    assert_eq!(true, account.external_id.is_none());
    assert_eq!(true, account.external_hash.is_none());
    assert_eq!(true, account.external_last_updated.is_none());
}

#[test]
fn get_account_details_by_id_with_external_details() {
    let db = initialize_test_database().unwrap();
    reset_db(&db).expect("Cant reset");

    let expected = create_new_account("AA Record", "9284", &12, &60, "SHA1", &db).unwrap();
    let _ = set_remote_account(
        &db,
        &expected,
        &Record {
            id: 15,
            sync_hash: "15HA482".to_string(),
            updated_at: 1847,
        },
    );

    let result = get_account_details_by_id(expected.id as u32, &db);

    assert_eq!(true, result.is_ok());

    let account = result.unwrap();

    assert_eq!(expected.id, account.id);
    assert_eq!("AA Record", account.name);
    assert_eq!("9284", account.secret);
    assert_eq!(12, account.otp_digits);
    assert_eq!(60, account.totp_step);
    assert_eq!(true, account.algorithm.is_some());
    assert_eq!(SHA1, account.algorithm.unwrap());
    assert_eq!(true, account.external_id.is_some());
    assert_eq!(15, account.external_id.unwrap());
    assert_eq!(true, account.external_hash.is_some());
    assert_eq!("15HA482", account.external_hash.unwrap());
    assert_eq!(true, account.external_last_updated.is_some());
    assert_eq!(1847, account.external_last_updated.unwrap());
}

#[test]
fn get_account_details_by_id_missing() {
    let db = initialize_test_database().unwrap();
    reset_db(&db).expect("Cant reset");

    let result = get_account_details_by_id(208, &db);

    assert_eq!(true, result.is_ok());

    assert_eq!(0, result.unwrap().id);
}

#[test]
fn delete_account_soft_delete() {
    let db = initialize_test_database().unwrap();
    reset_db(&db).expect("Cant reset");

    let _ = create_sync_account("User", "passowrd", "https://test.com", &db);

    let expected = create_new_account("AA Record", "9284", &12, &60, "SHA1", &db).unwrap();
    let _ = set_remote_account(
        &db,
        &expected,
        &Record {
            id: 15,
            sync_hash: "15HA482".to_string(),
            updated_at: 1847,
        },
    );

    let result = get_account_details_by_id(expected.id as u32, &db).unwrap();

    let delete_result = delete_account(&result, &db);

    assert_eq!(true, delete_result.is_ok());
    assert_eq!(true, delete_result.unwrap());

    let soft_delete_result = get_soft_deleted_accounts(&db);

    assert_eq!(true, soft_delete_result.is_ok());

    let soft_deletes = soft_delete_result.unwrap();

    assert_eq!(1, soft_deletes.len());
    assert_eq!(result.id, soft_deletes[0].id);
}

#[test]
fn delete_account_hard() {
    let db = initialize_test_database().unwrap();
    reset_db(&db).expect("Cant reset");

    let expected = create_new_account("AA Record", "9284", &12, &60, "SHA1", &db).unwrap();

    let result = get_account_details_by_id(expected.id as u32, &db).unwrap();

    let delete_result = delete_account(&result, &db);

    assert_eq!(true, delete_result.is_ok());
    assert_eq!(true, delete_result.unwrap());

    let result = get_account_details_by_id(expected.id as u32, &db).unwrap();

    assert_eq!(0, result.id);
}

#[test]
fn set_sync_accounts_success() {
    let db = initialize_test_database().unwrap();
    reset_db(&db).expect("Cant reset");

    let result = create_sync_account("User", "passowrd", "https://test.com", &db);

    assert_eq!(true, result.is_ok());

    let account = result.unwrap();

    assert_eq!(1, account.id);
    assert_eq!("User", account.username);
    assert_eq!("passowrd", account.password);
    assert_eq!("https://test.com", account.url);
}

#[test]
fn update_sync_accounts() {
    let db = initialize_test_database().unwrap();
    reset_db(&db).expect("Cant reset");

    let original = create_sync_account("User", "passowrd", "https://test.com", &db).unwrap();

    assert_eq!(true, original.id != 0);

    let update = SyncAccount {
        id: original.id.clone(),
        username: "updated".to_string(),
        password: "wjshf".to_string(),
        url: "http://updated.com".to_string(),
        token: None,
    };

    let result = update_sync_account(update, &db).unwrap();

    assert_eq!(true, result);

    let final_account = get_main_sync_account(&db).unwrap();

    assert_eq!(1, final_account.id);
    assert_eq!("updated", final_account.username);
    assert_eq!("wjshf", final_account.password);
    assert_eq!("http://updated.com", final_account.url);
}

#[test]
fn delete_sync_accounts() {
    let db = initialize_test_database().unwrap();
    reset_db(&db).expect("Cant reset");

    let account = create_sync_account("User", "password", "https://test.com", &db).unwrap();

    let result = delete_sync_account(account.id, &db).unwrap();

    assert_eq!(true, result);

    let account = get_main_sync_account(&db).unwrap();

    assert_eq!(0, account.id);
}

#[test]
fn error_sync_log() {
    let db = initialize_test_database().unwrap();
    reset_db(&db).expect("Cant reset");

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Could not generate UNIX time");
    let timestamp_before = since_the_epoch.as_secs();

    let result = create_sync_log(&db, "Error Test".to_string(), SyncLogType::ERROR);

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Could not generate UNIX time");
    let timestamp_after = since_the_epoch.as_secs();

    assert_eq!(true, result.is_ok());

    let sync_log = result.unwrap();

    assert_eq!("Error Test".to_string(), sync_log.log);
    assert_eq!(SyncLogType::ERROR, sync_log.log_type);
    assert_eq!(
        true,
        sync_log.timestamp >= timestamp_before && sync_log.timestamp <= timestamp_after
    );

    let get_result = get_sync_logs(&db);

    assert_eq!(true, get_result.is_ok());

    let sync_logs = get_result.unwrap();

    assert_eq!(1, sync_logs.len());

    assert_eq!("Error Test".to_string(), sync_logs[0].log);
    assert_eq!(SyncLogType::ERROR, sync_logs[0].log_type);
    assert_eq!(
        true,
        sync_logs[0].timestamp >= timestamp_before && sync_logs[0].timestamp <= timestamp_after
    );
}

fn initialize_test_database() -> Result<Connection, rusqlite::Error> {
    let base_path = PathBuf::from("./bin");
    let sqlite_path = base_path.join(SQLITE_TEST_NAME);

    let encryption_path = PathBuf::from("./bin");

    initialize_database(sqlite_path, encryption_path)
}

fn reset_db(db: &Connection) -> Result<(), rusqlite::Error> {
    db.prepare("DELETE FROM accounts")?.execute([])?;
    db.prepare("DELETE FROM sync_accounts")?.execute([])?;
    db.prepare("DELETE FROM sync_logs")?.execute([])?;

    Ok(())
}