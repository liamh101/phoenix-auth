// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;
mod encryption;
mod otp_parser;
mod otp_exporter;
mod sync;

use libotp::{totp, totp_override};
use state::{AppState};
use tauri::{State, Manager, AppHandle};
use crate::database::SyncAccount;
use crate::otp_exporter::account_to_url;
use crate::otp_parser::{is_valid_url, parse_url};
use crate::state::ServiceAccess;
use crate::sync::{get_jwt_token, get_record, get_single_record};


#[tauri::command]
fn get_one_time_password_for_account(app_handle: AppHandle, account: u32) -> String {
    let account = app_handle.db(|db| database::get_account_details_by_id(account, db)).unwrap();

    if account.id == 0 {
        return "Failed to generate OTP".to_string()
    }

    let decrypted_secret = encryption::decrypt(&account.secret);

    if account.algorithm.is_some() {
        return match totp_override(&decrypted_secret, account.otp_digits as u32, account.totp_step as u64, 0, account.algorithm.unwrap().to_hotp_algorithm()) {
            Some(otp) => {
                otp.to_string()
            },
            None => {
                "Failed to generate OTP".to_string()
            }
        }
    }

    match totp(&decrypted_secret, account.otp_digits as u32, account.totp_step as u64, 0) {
        Some(otp) => {
            otp.to_string()
        },
        None => {
            "Failed to generate OTP".to_string()
        }
    }
}

#[tauri::command]
fn create_new_account(app_handle: AppHandle, name: &str, secret: &str, digits: i32, step: i32, algorithm: &str) -> String {
    let account_exists = app_handle.db(|db| database::account_name_exists(name, db)).unwrap();

    if account_exists {
        return format!("Account already exists: {}", name)
    }

    if totp(secret, digits as u32, step as u64, 0).is_none() {
        return "Invalid 2FA Secret".to_string()
    }

    let encryption_secret = encryption::encrypt(secret);

    app_handle.db(|db| database::create_new_account(name, &encryption_secret, &digits, &step, algorithm, db)).unwrap();

    format!("Created account called: {}", name)
}

#[tauri::command]
fn get_all_accounts(app_handle: AppHandle, filter: &str) -> String {
    let accounts = app_handle.db(|db| database::get_all_accounts(db, filter)).unwrap();

    match serde_json::to_string(&accounts) {
        Ok(result) => result,
        _ => "{\"Error\": \"Can't get accounts\"}".to_string()
    }
}

#[tauri::command]
fn delete_account(app_handle: AppHandle, account_id: u32) -> String {
    let account = app_handle.db(|db| database::get_account_details_by_id(account_id, db)).unwrap();
    let result = app_handle.db(|db| database::delete_account(account, db)).unwrap();

    match result {
        true => "Success".to_string(),
        false => "Failure".to_string(),
    }
}

#[tauri::command]
fn parse_otp_url(otp_url: &str) -> String {
    if !is_valid_url(otp_url) {
        return "{\"Error\": \"Invalid OTP URL\"}".to_string()
    }

    let account = parse_url(otp_url);

    match serde_json::to_string(&account) {
        Ok(result) => result,
        _ => "{\"Error\": \"Can't create account\"}".to_string()
    }
}

#[tauri::command]
fn export_accounts_to_wa(app_handle: AppHandle) -> String {
    let base_accounts = app_handle.db(|db| database::get_all_accounts(db, "")).unwrap();
    let mut otps: String = "".to_owned();

    for base_account in base_accounts {
        let verbose_account = app_handle.db(|db| database::get_account_details_by_id(base_account.id as u32, db)).unwrap();
        let url = account_to_url(verbose_account);

        otps.push_str(&url);
        otps.push_str("\n");
    }

    return otps
}

#[tauri::command]
async fn validate_sync_account(host: &str, username: &str, password: &str) -> Result<String, String> {
    let token_response = get_jwt_token(host, username, password).await;
    match token_response {
        Ok(token) => Ok(token),
        Err(e) => Err(e.formatted_message()),
    }
}

#[tauri::command]
fn save_sync_account(host: &str, username: &str, password: &str, app_handle: AppHandle) -> Result<SyncAccount, ()> {
    let existing_account = app_handle.db(|db| database::get_main_sync_account(db)).unwrap();

    if existing_account.id == 0 {
        let new_account = app_handle.db(|db| database::create_sync_account(username, password, host, db)).unwrap();
        return Ok(new_account);
    }

    let updated_sync_account = SyncAccount {
        id: existing_account.id,
        username: username.to_string(),
        password: password.to_string(),
        url: host.to_string(),
        token: None,
    };

    app_handle.db(|db| database::update_sync_account(updated_sync_account, db)).unwrap();
    Ok(SyncAccount {
        id: existing_account.id,
        username: username.to_string(),
        password: password.to_string(),
        url: host.to_string(),
        token: None,
    })
}

#[tauri::command]
fn get_existing_sync_account(app_handle: AppHandle) -> Result<SyncAccount, String> {
    let existing_account = app_handle.db(|db| database::get_main_sync_account(db)).unwrap();

    if existing_account.id == 0 {
        return Err("Sync Account does not exist".to_string());
    }

    return Ok(
        SyncAccount {
            id: existing_account.id,
            username: existing_account.username,
            password: "".to_string(),
            url: existing_account.url,
            token: None,
        }
    );
}

async fn sync_all_accounts(app_handle: AppHandle, sync_account: SyncAccount) {
    let authenticated_account = sync::authenticate_account(sync_account.clone()).await.unwrap();

    let accounts_without_external = app_handle.db(|db| database::get_accounts_without_external_id(db)).unwrap();
    let manifest_result = sync::get_manifest(&authenticated_account).await;

    let manifest = match manifest_result {
        Ok(manifest) => manifest,
        Err(_) => return
    };

    //Loop through accounts anything missing sync details, grab, anything that's out of date, grab, anything remaining in manifest, grab

    for account in accounts_without_external {
        if account.external_id.is_none() {
            let full_account_details = app_handle.db(|db| database::get_account_details_by_id(account.id as u32, &db)).unwrap();

            let record = match get_record(&full_account_details, &authenticated_account).await {
                Ok(record) => record,
                Err(_err) => continue,
            };
            app_handle.db(|db| database::set_remote_account(db, &account, &record)).unwrap();
            continue;
        }
    }

    let mut manifest_ids = Vec::new();

    for manifest_item in manifest {
        let potential_account = app_handle.db(|db| database::get_account_by_external_id(&manifest_item.id, &db)).unwrap();

        //Log manifest id to check what items need removing
        manifest_ids.push(manifest_item.id);

        if potential_account.is_none() {
            //Get external and create
            let new_account_record = get_single_record(&manifest_item.id, &sync_account).await.unwrap();
            let mut new_account_algo = "".to_string();

            if new_account_record.algorithm.is_some() {
                new_account_algo = new_account_record.algorithm.clone().unwrap().algorithm_to_string();
            }

            let new_account = app_handle.db(|db| database::create_new_account(&new_account_record.name, &new_account_record.secret, &new_account_record.otpDigits, &new_account_record.totpStep, &new_account_algo, &db)).unwrap();
            app_handle.db(|db| database::set_remote_account(db, &new_account, &new_account_record.to_record())).unwrap();
            continue;
        }

        let account = potential_account.unwrap();

        if account.external_last_updated.unwrap() < manifest_item.updatedAt {
            let existing_record = get_single_record(&manifest_item.id, &sync_account).await.unwrap();
            let mut new_account_algo = "".to_string();

            if existing_record.algorithm.is_some() {
                new_account_algo = existing_record.algorithm.clone().unwrap().algorithm_to_string();
            }

            app_handle.db(|db| database::update_existing_account(&account.id, &existing_record.name, &existing_record.secret, existing_record.otpDigits, existing_record.totpStep, &new_account_algo, &db)).unwrap();
            app_handle.db(|db| database::set_remote_account(db, &account, &existing_record.to_record())).unwrap();
            continue
        }
    }

    //Remove accounts not in manifest list
    app_handle.db(|db| database::delete_accounts_without_external_ids(manifest_ids, db)).unwrap();
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { db: Default::default() })
        .invoke_handler(tauri::generate_handler![
            create_new_account,
            get_all_accounts,
            delete_account,
            get_one_time_password_for_account,
            parse_otp_url,
            export_accounts_to_wa,
            validate_sync_account,
            save_sync_account,
            get_existing_sync_account
        ])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = database::initialize_database(&handle).expect("Database initialize should succeed");
            rusqlite::vtab::array::load_module(&db)?;
            let sync_account = database::get_main_sync_account(&db).unwrap();

            *app_state.db.lock().unwrap() = Some(db);

            if sync_account.id != 0 {
                tauri::async_runtime::spawn(sync_all_accounts(handle, sync_account));
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
