// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;
mod encryption;

use libotp::totp;
use state::{AppState};
use tauri::{State, Manager, AppHandle};
use crate::state::ServiceAccess;

const TOTP_STEP: u64 = 30;
const OTP_DIGITS: u32 = 6;

#[tauri::command]
fn get_one_time_password_for_account(app_handle: AppHandle, account: u32) -> String {
    let account = app_handle.db(|db| database::get_account_details_by_id(account, db)).unwrap();
    let decrypted_secret = encryption::decrypt(&account.secret);

    match totp(&decrypted_secret, OTP_DIGITS, TOTP_STEP, 0) {
        Some(otp) => {
            format!("{}", otp)
        },
        None => {
            "Failed to generate OTP".to_string()
        }
    }
}

#[tauri::command]
fn create_new_account(app_handle: AppHandle, name: &str, secret: &str) -> String {
    let account_exists = app_handle.db(|db| database::account_name_exists(name, db)).unwrap();

    if account_exists {
        return format!("Account already exists: {}", name)
    }

    if totp(secret, OTP_DIGITS, TOTP_STEP, 0) == None {
        return "Invalid 2FA Secret".to_string()
    }

    let encryption_secret = encryption::encrypt(secret);

    app_handle.db(|db| database::create_new_account(name, &encryption_secret, db)).unwrap();

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

fn main() {
    tauri::Builder::default()
        .manage(AppState { db: Default::default() })
        .invoke_handler(tauri::generate_handler![create_new_account, get_all_accounts, get_one_time_password_for_account])
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
