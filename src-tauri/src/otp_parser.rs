use tauri::regex::Regex;
use urlencoding::decode;
use crate::database::{Account};

const IDENTIFIER_LIMIT: usize = 255;

pub fn is_valid_url(url: &str) -> bool {
    let re = Regex::new(r"^otpauth:\/\/([ht]otp)\/(?:[a-zA-Z0-9%]+:)?([^?]+)\?(.*secret).*").unwrap();

    re.is_match(url)
}

pub fn parse_url(url: &str) -> Account {
    Account {
        id: 0,
        name: get_identifier(url),
        secret: get_secret(url),
        totp_step: 30,
        otp_digits: get_digits(url),
        algorithm: None,
    }
}

fn get_identifier(url: &str) -> String {
    let name_re = Regex::new(r"otpauth:\/\/(totp|hotp)\/(?<identity>.+?)\?").unwrap();
    let Some(name) = name_re.captures(url) else { return "Unidentified".to_string()};


    let decoded_name = decode(&name["identity"]).unwrap().to_string();


    if decoded_name.len() > IDENTIFIER_LIMIT {
        return (&decoded_name[..IDENTIFIER_LIMIT]).parse().unwrap()
    }

    return decoded_name
}

fn get_secret(url: &str) -> String {
    let secret_re = Regex::new(r"((&|\?)secret=)(?<secret>.+?)(&|$)").unwrap();
    let Some(secret) = secret_re.captures(url) else { return "".to_string()};

    secret["secret"].to_string()
}

fn get_digits(url: &str) -> i32 {
    let digits_re = Regex::new(r"((&|\?)digits=)(?<digits>.+?)(&|$)").unwrap();

    let Some(digits) = digits_re.captures(url) else { return 6};

    (&digits["digits"]).parse().unwrap()
}