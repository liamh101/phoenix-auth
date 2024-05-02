use tauri::regex::Regex;
use urlencoding::decode;
use crate::database::{Account};

//TODO - Missing Algorithm Parse
// Missing Period Parse

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


#[cfg(test)]
mod tests {
    use crate::otp_parser::{is_valid_url, parse_url};

    #[test]
    fn test_valid_otp_url_totp_full() {
        assert_eq!(true, is_valid_url("otpauth://totp/TestOne?digits=6&secret=H3LL0W0RLD"));
        assert_eq!(true, is_valid_url("otpauth://totp/TestOne?secret=H3LL0W0RLD&digits=6"));
    }

    #[test]
    fn test_valid_otp_url_totp_no_digits() {
        assert_eq!(true, is_valid_url("otpauth://totp/TestOne?secret=H3LL0W0RLD"))
    }

    #[test]
    fn test_invalid_otp_url_totp_missing_secret() {
        assert_eq!(false, is_valid_url("otpauth://totp/TestOne?digits=6"))
    }

    #[test]
    fn test_valid_otp_url_hotp_full() {
        assert_eq!(true, is_valid_url("otpauth://hotp/TestOne?digits=6&secret=H3LL0W0RLD"))
    }

    #[test]
    fn test_parse_url_full_totp() {
        let account = parse_url("otpauth://totp/TestOne?digits=8&secret=H3LL0W0RLD");

        assert_eq!(0, account.id);
        assert_eq!("TestOne", account.name);
        assert_eq!("H3LL0W0RLD", account.secret);
        assert_eq!(8, account.otp_digits);
        assert_eq!(30, account.totp_step);
    }

    #[test]
    fn test_parse_url_totp_parse_url() {
        let account = parse_url("otpauth://totp/URL%20Parse%20-%20Test%20%28authenticator%29?secret=H3LL0W0RLD&digits=8");

        assert_eq!(0, account.id);
        assert_eq!("URL Parse - Test (authenticator)", account.name);
        assert_eq!("H3LL0W0RLD", account.secret);
        assert_eq!(8, account.otp_digits);
        assert_eq!(30, account.totp_step);
    }


    #[test]
    fn test_parse_url_missing_digits() {
        let account = parse_url("otpauth://totp/TestOne?secret=H3LL0W0RLD");

        assert_eq!(0, account.id);
        assert_eq!("TestOne", account.name);
        assert_eq!("H3LL0W0RLD", account.secret);
        assert_eq!(6, account.otp_digits);
        assert_eq!(30, account.totp_step);
    }
}