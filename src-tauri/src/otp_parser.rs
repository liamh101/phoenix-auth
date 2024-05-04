use tauri::regex::Regex;
use urlencoding::decode;
use crate::database::{Account, AccountAlgorithm};

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
        totp_step: get_period(url),
        otp_digits: get_digits(url),
        algorithm: get_algorithm(url),
    }
}

fn get_identifier(url: &str) -> String {
    let name_re = Regex::new(r"otpauth:\/\/(totp|hotp)\/(?<identity>.+?)\?").unwrap();
    let Some(name) = name_re.captures(url) else { return "Unidentified".to_string()};


    let decoded_name = decode(&name["identity"]).unwrap().to_string();


    if decoded_name.len() > IDENTIFIER_LIMIT {
        return decoded_name[..IDENTIFIER_LIMIT].parse().unwrap()
    }

    decoded_name
}

fn get_secret(url: &str) -> String {
    let secret_re = Regex::new(r"((&|\?)secret=)(?<secret>.+?)(&|$)").unwrap();
    let Some(secret) = secret_re.captures(url) else { return "".to_string()};

    secret["secret"].to_string()
}

fn get_digits(url: &str) -> i32 {
    let digits_re = Regex::new(r"((&|\?)digits=)(?<digits>.+?)(&|$)").unwrap();

    let Some(digits) = digits_re.captures(url) else { return 6};

    digits["digits"].parse().unwrap()
}

fn get_algorithm(url: &str) -> Option<AccountAlgorithm> {
    let algorithm_re = Regex::new(r"((&|\?)algorithm=)(?<algorithm>.+?)(&|$)").unwrap();

    let algorithm = algorithm_re.captures(url)?;

    AccountAlgorithm::string_to_algorithm(algorithm["algorithm"].to_string())
}

fn get_period(url: &str) -> i32 {
    let period_re = Regex::new(r"((&|\?)period=)(?<period>.+?)(&|$)").unwrap();

    let Some(period) = period_re.captures(url) else { return 30};

    period["period"].parse().unwrap()
}


#[cfg(test)]
mod tests {
    use crate::database::AccountAlgorithm;
    use crate::otp_parser::{is_valid_url, parse_url};

    #[test]
    fn test_valid_otp_url_totp_full() {
        assert_eq!(true, is_valid_url("otpauth://totp/TestOne?digits=6&secret=H3LL0W0RLD&algorithm=SHA1&period=30"));
        assert_eq!(true, is_valid_url("otpauth://totp/TestOne?secret=H3LL0W0RLD&algorithm=SHA1&period=30&digits=6"));
        assert_eq!(true, is_valid_url("otpauth://totp/TestOne?algorithm=SHA1&period=30&secret=H3LL0W0RLD&digits=6"));
        assert_eq!(true, is_valid_url("otpauth://totp/TestOne?period=30&algorithm=SHA1&secret=H3LL0W0RLD&digits=6"));
    }

    #[test]
    fn test_valid_otp_url_totp_no_digits_or_algorithm_or_period() {
        assert_eq!(true, is_valid_url("otpauth://totp/TestOne?secret=H3LL0W0RLD"))
    }

    #[test]
    fn test_invalid_otp_url_totp_missing_secret() {
        assert_eq!(false, is_valid_url("otpauth://totp/TestOne?digits=6"))
    }

    #[test]
    fn test_valid_otp_url_hotp_full() {
        assert_eq!(true, is_valid_url("otpauth://hotp/TestOne?digits=6&secret=H3LL0W0RLD&algorithm=SHA1&period=30"))
    }

    #[test]
    fn test_parse_url_full_totp() {
        let account = parse_url("otpauth://totp/TestOne?digits=8&secret=H3LL0W0RLD&algorithm=SHA1&period=60");

        assert_eq!(0, account.id);
        assert_eq!("TestOne", account.name);
        assert_eq!("H3LL0W0RLD", account.secret);
        assert_eq!(8, account.otp_digits);
        assert_eq!(60, account.totp_step);
        assert_eq!(Option::from(AccountAlgorithm::SHA1), account.algorithm);
    }

    #[test]
    fn test_parse_url_totp_parse_url() {
        let account = parse_url("otpauth://totp/URL%20Parse%20-%20Test%20%28authenticator%29?secret=H3LL0W0RLD&digits=8&algorithm=SHA256&period=120");

        assert_eq!(0, account.id);
        assert_eq!("URL Parse - Test (authenticator)", account.name);
        assert_eq!("H3LL0W0RLD", account.secret);
        assert_eq!(8, account.otp_digits);
        assert_eq!(120, account.totp_step);
        assert_eq!(Option::from(AccountAlgorithm::SHA256), account.algorithm);
    }


    #[test]
    fn test_parse_url_missing_digits() {
        let account = parse_url("otpauth://totp/TestOne?secret=H3LL0W0RLD&algorithm=SHA512&period=60");

        assert_eq!(0, account.id);
        assert_eq!("TestOne", account.name);
        assert_eq!("H3LL0W0RLD", account.secret);
        assert_eq!(6, account.otp_digits);
        assert_eq!(60, account.totp_step);
        assert_eq!(Option::from(AccountAlgorithm::SHA512), account.algorithm);
    }

    #[test]
    fn test_parse_url_missing_algorithm() {
        let account = parse_url("otpauth://totp/URL%20Parse%20-%20Test%20%28authenticator%29?secret=H3LL0W0RLD&digits=8&period=60");

        assert_eq!(0, account.id);
        assert_eq!("URL Parse - Test (authenticator)", account.name);
        assert_eq!("H3LL0W0RLD", account.secret);
        assert_eq!(8, account.otp_digits);
        assert_eq!(60, account.totp_step);
        assert_eq!(None, account.algorithm);
    }

    #[test]
    fn test_parse_url_missing_period() {
        let account = parse_url("otpauth://totp/URL%20Parse%20-%20Test%20%28authenticator%29?secret=H3LL0W0RLD&digits=8&algorithm=SHA256");

        assert_eq!(0, account.id);
        assert_eq!("URL Parse - Test (authenticator)", account.name);
        assert_eq!("H3LL0W0RLD", account.secret);
        assert_eq!(8, account.otp_digits);
        assert_eq!(30, account.totp_step);
        assert_eq!(Option::from(AccountAlgorithm::SHA256), account.algorithm);
    }

    #[test]
    fn test_parse_url_invalid_algorithm() {
        let account = parse_url("otpauth://totp/URL%20Parse%20-%20Test%20%28authenticator%29?secret=H3LL0W0RLD&digits=8&algorithm=Hello");

        assert_eq!(0, account.id);
        assert_eq!("URL Parse - Test (authenticator)", account.name);
        assert_eq!("H3LL0W0RLD", account.secret);
        assert_eq!(8, account.otp_digits);
        assert_eq!(30, account.totp_step);
        assert_eq!(None, account.algorithm);
    }
}