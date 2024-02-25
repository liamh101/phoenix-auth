// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use libotp::totp;

const TOTP_STEP: u64 = 30;
const OTP_DIGITS: u32 = 6;

#[tauri::command]
fn get_one_time_password(secret: &str) -> String {
    match totp(secret, OTP_DIGITS, TOTP_STEP, 0) {
        Some(otp) => {
            format!("OTP: {}!", otp)
        },
        None => {
            format!("Failed to generate OTP")
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_one_time_password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
