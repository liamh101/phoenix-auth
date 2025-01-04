use crate::database::{Account, AccountAlgorithm};
use crate::encryption;
use urlencoding::encode;

pub fn account_to_url(account: Account) -> String {
    "otpauth://totp/".to_owned()
        + &encode(&account.name)
        + &get_secret(&account)
        + &get_period(&account)
        + &get_digits(&account)
        + &get_algorithm(&account)
}

fn get_secret(account: &Account) -> String {
    "?secret=".to_owned() + &encryption::decrypt(&account.secret)
}

fn get_period(account: &Account) -> String {
    "&period=".to_owned() + &account.totp_step.to_string()
}

fn get_digits(account: &Account) -> String {
    "&digits=".to_owned() + &account.otp_digits.to_string()
}

fn get_algorithm(account: &Account) -> String {
    if account.algorithm.is_none() {
        return "".to_string();
    }

    let algorithm = <std::option::Option<AccountAlgorithm> as Clone>::clone(&account.algorithm)
        .unwrap()
        .algorithm_to_string();

    "&algorithm=".to_string() + &algorithm
}

#[cfg(test)]
mod tests {
    use crate::database::{Account, AccountAlgorithm};
    use crate::encryption;
    use crate::otp_exporter::account_to_url;

    #[test]
    fn test_full_account_sha1() {
        let account = Account {
            id: 14,
            name: "Hello World".to_string(),
            secret: encryption::encrypt("123dhahgs").to_string(),
            totp_step: 30,
            otp_digits: 8,
            algorithm: Option::from(AccountAlgorithm::SHA1),
            external_id: None,
            external_last_updated: None,
            external_hash: None,
            deleted_at: None,
        };

        let result = account_to_url(account);

        assert_eq!(
            "otpauth://totp/Hello%20World?secret=123dhahgs&period=30&digits=8&algorithm=SHA1"
                .to_string(),
            result
        );
    }

    #[test]
    fn test_full_account_sha256() {
        let account = Account {
            id: 12,
            name: "Test".to_string(),
            secret: encryption::encrypt("bingoTest").to_string(),
            totp_step: 60,
            otp_digits: 6,
            algorithm: Option::from(AccountAlgorithm::SHA256),
            external_id: None,
            external_last_updated: None,
            external_hash: None,
            deleted_at: None,
        };

        let result = account_to_url(account);

        assert_eq!(
            "otpauth://totp/Test?secret=bingoTest&period=60&digits=6&algorithm=SHA256".to_string(),
            result
        );
    }

    #[test]
    fn test_full_account_sha512() {
        let account = Account {
            id: 1,
            name: "Hello?!".to_string(),
            secret: encryption::encrypt("bingoTest").to_string(),
            totp_step: 90,
            otp_digits: 9,
            algorithm: Option::from(AccountAlgorithm::SHA512),
            external_id: None,
            external_last_updated: None,
            external_hash: None,
            deleted_at: None,
        };

        let result = account_to_url(account);

        assert_eq!(
            "otpauth://totp/Hello%3F%21?secret=bingoTest&period=90&digits=9&algorithm=SHA512"
                .to_string(),
            result
        );
    }

    #[test]
    fn test_missing_algorithm() {
        let account = Account {
            id: 1,
            name: "Hello?!".to_string(),
            secret: encryption::encrypt("bingoTest").to_string(),
            totp_step: 90,
            otp_digits: 9,
            algorithm: None,
            external_id: None,
            external_last_updated: None,
            external_hash: None,
            deleted_at: None,
        };

        let result = account_to_url(account);

        assert_eq!(
            "otpauth://totp/Hello%3F%21?secret=bingoTest&period=90&digits=9".to_string(),
            result
        );
    }
}
