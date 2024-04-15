use rusqlite::{Connection, named_params};
use tauri::AppHandle;
use std::fs;
use libotp::HOTPAlgorithm;
use serde::{Deserialize, Serialize};

const SQLITE_NAME: &str = "Phoenix.sqlite";
const CURRENT_DB_VERSION: u32 = 2;

mod m2024_03_31_account_creation;
mod m2024_04_01_account_timeout_algorithm;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub secret: String,
    pub totp_step: i32,
    pub otp_digits: i32,
    pub algorithm: Option<AccountAlgorithm>
}

#[derive(Serialize, Deserialize)]
pub enum AccountAlgorithm {
    SHA1,
    SHA256,
    SHA512,
}

impl AccountAlgorithm {
    pub fn to_hotp_algorithm(&self) -> HOTPAlgorithm {
        match *self {
            AccountAlgorithm::SHA1 => HOTPAlgorithm::HMACSHA1,
            AccountAlgorithm::SHA256 => HOTPAlgorithm::HMACSHA256,
            AccountAlgorithm::SHA512 => HOTPAlgorithm::HMACSHA512,
        }
    }
}

pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle.path_resolver().app_data_dir().expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join(SQLITE_NAME);

    let mut db = Connection::open(sqlite_path)?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| { Ok(row.get(0)?) })?;
    drop(user_pragma);

    let _ = update_database(&mut db, existing_user_version);

    Ok(db)
}

pub fn create_new_account(name: &str, secret: &str, digits: i32, step: i32, algorithm: &str, db: &Connection) -> Result<(), rusqlite::Error>  {
    let mut statement = db.prepare("INSERT INTO accounts (name, secret, totp_step, otp_digits, totp_algorithm) VALUES (@name, @secret, @step, @digits, @algorithm)")?;

    if algorithm == "" {
        statement.execute(named_params! { "@name": name, "@secret": secret, "@step": step, "@digits": digits, "@algorithm": None::<&str>})?;

        return Ok(())
    }

    statement.execute(named_params! { "@name": name, "@secret": secret, "@step": step, "@digits": digits, "@algorithm": algorithm })?;

    Ok(())
}

pub fn get_all_accounts(db: &Connection, filter: &str) -> Result<Vec<Account>, rusqlite::Error>  {
    let mut statement = db.prepare("SELECT id, name, totp_step, otp_digits FROM accounts WHERE name LIKE ? ORDER BY name ASC")?;
    let mut rows = statement.query([ "%".to_owned() + filter + "%"])?;
    let mut items = Vec::new();

    while let Some(row) = rows.next()? {
        let title: Account = Account {id: row.get("id")?, name: row.get("name")?, secret: "".to_string(), totp_step: row.get("totp_step")?, otp_digits: row.get("otp_digits")?, algorithm: None };

        items.push(title);
    }

    Ok(items)
}

pub fn get_account_details_by_id(id: u32, db: &Connection) -> Result<Account, rusqlite::Error> {
    let mut statement = db.prepare("SELECT id, name, secret, totp_step, otp_digits, totp_algorithm FROM accounts WHERE id = ?")?;
    let mut rows = statement.query([id])?;

    match rows.next()? {
        Some(row) => {
            let algorithm = match row.get("totp_algorithm")? {
                Some(string_algorithm) => string_to_algorithm(string_algorithm),
                None => None
            };

            Ok(Account {id: row.get("id")?, name: row.get("name")?, secret: row.get("secret")?, totp_step: row.get("totp_step")?, otp_digits: row.get("otp_digits")?, algorithm })
        }
        _ => {
            Ok(Account {id: 0, name: "".to_string(), secret: "".to_string(), totp_step: 0, otp_digits: 0, algorithm: None })
        }
    }
}

fn string_to_algorithm(algorithm: String) -> Option<AccountAlgorithm> {
    match algorithm.as_str() {
        "SHA1" => Option::from(AccountAlgorithm::SHA1),
        "SHA256" => Option::from(AccountAlgorithm::SHA256),
        "SHA512" => Option::from(AccountAlgorithm::SHA512),
        _ => None
    }
}

pub fn delete_account(account: Account, db: &Connection) -> Result<bool,  rusqlite::Error> {
    let mut statement = db.prepare("DELETE FROM accounts WHERE id = ?")?;
    let affected_rows = statement.execute([account.id])?;

    Ok(affected_rows == 1)
}

pub fn account_name_exists(name: &str, db: &Connection) -> Result<bool, rusqlite::Error> {
    let mut statement = db.prepare("SELECT id, name, secret FROM accounts WHERE name = ?")?;
    let mut rows = statement.query([name])?;

    match rows.next()? {
        Some(_row) => {Ok(true)},
        _ => {Ok(false)}
    }
}

fn update_database(db: &mut Connection, existing_version: u32) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION {
        m2024_03_31_account_creation::migrate(db, existing_version).expect("FAILED: Account Table Creation - ");
        m2024_04_01_account_timeout_algorithm::migrate(db, existing_version).expect("FAILED: Account timeout algorithm - ");
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use libotp::HOTPAlgorithm;
    use crate::database::{AccountAlgorithm, string_to_algorithm};

    #[test]
    fn can_parse_sha1() {
        let _algorithm = string_to_algorithm("SHA1".to_string()).unwrap();
        let is_valid = matches!(AccountAlgorithm::SHA1, _algorithm);

        assert_eq!(true, is_valid);
    }

    #[test]
    fn can_parse_sha256() {
        let _algorithm = string_to_algorithm("SHA256".to_string()).unwrap();
        let is_valid = matches!(AccountAlgorithm::SHA256, _algorithm);

        assert_eq!(true, is_valid);
    }

    #[test]
    fn can_parse_sha512() {
        let _algorithm = string_to_algorithm("SHA512".to_string()).unwrap();
        let is_valid = matches!(AccountAlgorithm::SHA512, _algorithm);

        assert_eq!(true, is_valid);
    }

    #[test]
    fn can_parse_invalid() {
        let is_none = string_to_algorithm("Hello world".to_string()).is_none();

        assert_eq!(true, is_none);
    }

    #[test]
    fn can_translate_sha1() {
        assert_eq!(true, matches!(AccountAlgorithm::SHA1.to_hotp_algorithm(), HOTPAlgorithm::HMACSHA1));
    }

    #[test]
    fn can_translate_sha256() {
        assert_eq!(true, matches!(AccountAlgorithm::SHA256.to_hotp_algorithm(), HOTPAlgorithm::HMACSHA256));
    }

    #[test]
    fn can_translate_sha512() {
        assert_eq!(true, matches!(AccountAlgorithm::SHA512.to_hotp_algorithm(), HOTPAlgorithm::HMACSHA512));
    }
}