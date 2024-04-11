// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;
mod encryption;

use libotp::{totp, totp_override};
use state::{AppState};
use tauri::{State, Manager, AppHandle};
use crate::state::ServiceAccess;

const TOTP_STEP: u64 = 30;
const OTP_DIGITS: u32 = 6;

#[tauri::command]
fn get_one_time_password_for_account(app_handle: AppHandle, account: u32) -> String {
    let account = app_handle.db(|db| database::get_account_details_by_id(account, db)).unwrap();
    let decrypted_secret = encryption::decrypt(&account.secret);

    if account.algorithm.is_some() {
        return match totp_override(&decrypted_secret, account.otp_digits as u32, account.totp_step as u64, 0, account.algorithm.unwrap().to_hotp_algorithm()) {
            Some(otp) => {
                format!("{}", otp)
            },
            None => {
                "Failed to generate OTP".to_string()
            }
        }
    }

    match totp(&decrypted_secret, account.otp_digits as u32, account.totp_step as u64, 0) {
        Some(otp) => {
            format!("{}", otp)
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

    if totp(secret, digits as u32, step as u64, 0) == None {
        return "Invalid 2FA Secret".to_string()
    }

    let encryption_secret = encryption::encrypt(secret);

    app_handle.db(|db| database::create_new_account(name, &encryption_secret, digits, step, algorithm, db)).unwrap();

    format!("Created account called: {}", name)
}

#[tauri::command]
fn get_all_accounts(app_handle: AppHandle, filter: &str) -> String {
    let accounts = app_handle.db(|db| database::get_all_accounts(db, filter)).unwrap();

    match serde_json::to_string(&accounts) {
        Ok(result) => result,
        _ => format!("{{\"Error\": \"Can't get accounts\"}}")
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

fn main() {
    tauri::Builder::default()
        .manage(AppState { db: Default::default() })
        .invoke_handler(tauri::generate_handler![create_new_account, get_all_accounts, delete_account, get_one_time_password_for_account])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = database::initialize_database(&handle).expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
