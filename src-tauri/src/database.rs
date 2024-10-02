use rusqlite::{Connection, named_params};
use tauri::AppHandle;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use libotp::HOTPAlgorithm;
use rusqlite::types::Value;
use serde::{Deserialize, Serialize};
use crate::encryption::{decrypt, encrypt};
use crate::sync_api::Record;

const SQLITE_NAME: &str = "Phoenix.sqlite";
const SQLITE_TEST_NAME: &str = "Phoenix_test.sqlite";
const CURRENT_DB_VERSION: u32 = 6;

mod m2024_03_31_account_creation;
mod m2024_04_01_account_timeout_algorithm;
mod m2024_07_01_sync_account_creation;
mod m2024_07_15_account_sync_details;
mod m2024_09_13_soft_delete_accounts;
mod m2024_09_15_remove_sync_error_log;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub secret: String,
    pub totp_step: i32,
    pub otp_digits: i32,
    pub algorithm: Option<AccountAlgorithm>,
    pub external_id: Option<i32>,
    pub external_last_updated: Option<u64>,
    pub external_hash: Option<String>,
    pub deleted_at: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

    pub fn string_to_algorithm(string: String) -> Option<AccountAlgorithm> {
        match string.as_str() {
            "SHA1" => Option::from(AccountAlgorithm::SHA1),
            "SHA256" => Option::from(AccountAlgorithm::SHA256),
            "SHA512" => Option::from(AccountAlgorithm::SHA512),
            _ => None
        }
    }

    pub fn algorithm_to_string(&self) -> String {
        match *self {
            AccountAlgorithm::SHA1 => "SHA1".to_owned(),
            AccountAlgorithm::SHA256 => "SHA256".to_owned(),
            AccountAlgorithm::SHA512 => "SHA512".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncAccount {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub url: String,
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SyncLogType {
    ERROR = 1
}

impl SyncLogType {
    pub fn u16_to_sync_log(sync_log: u16) -> Option<SyncLogType> {
        match sync_log {
            1 => Option::from(SyncLogType::ERROR),
            _ => None
        }
    }

    pub fn sync_log_to_u16(sync_log_type: SyncLogType) -> Option<u16> {
        match sync_log_type {
            SyncLogType::ERROR => Option::from(1),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SyncLog {
    pub id: i32,
    pub log: String,
    pub log_type: SyncLogType,
    pub timestamp: u64,
}


pub fn initialize_prod_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle.path_resolver().app_data_dir().expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join(SQLITE_NAME);

    initialize_database(sqlite_path)
}

pub fn initialize_test_database() -> Result<Connection, rusqlite::Error> {
    let base_path = PathBuf::from("./../");
    let sqlite_path = base_path.join(SQLITE_TEST_NAME);

    initialize_database(sqlite_path)
}

fn initialize_database(path: PathBuf) -> Result<Connection, rusqlite::Error> {
    let mut db = Connection::open(path)?;
    rusqlite::vtab::array::load_module(&db)?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| { row.get(0) })?;
    drop(user_pragma);

    let _ = update_database(&mut db, existing_user_version);

    Ok(db)
}

pub fn create_new_account(name: &str, secret: &str, digits: &i32, step: &i32, algorithm: &str, db: &Connection) -> Result<Account, rusqlite::Error>  {
    let mut insert_statement = db.prepare("INSERT INTO accounts (name, secret, totp_step, otp_digits, totp_algorithm) VALUES (@name, @secret, @step, @digits, @algorithm)")?;
    let mut get_statement = db.prepare("SELECT id, name, secret, totp_step, otp_digits, totp_algorithm FROM accounts WHERE name = @name AND secret = @secret")?;
    let mut final_algorithm = None::<&str>;

    if algorithm != "" {
        final_algorithm = Some(algorithm);
    }

    insert_statement.execute(named_params! { "@name": name, "@secret": secret, "@step": step, "@digits": digits, "@algorithm": final_algorithm })?;
    let mut rows = get_statement.query(named_params! {"@name": name, "@secret": secret})?;

    match rows.next()? {
        Some(row) => {
            let algorithm = match row.get("totp_algorithm")? {
                Some(string_algorithm) => AccountAlgorithm::string_to_algorithm(string_algorithm),
                None => None
            };

            Ok(Account {id: row.get("id")?, name: row.get("name")?, secret: row.get("secret")?, totp_step: row.get("totp_step")?, otp_digits: row.get("otp_digits")?, algorithm, external_id: None, external_last_updated: None, external_hash: None, deleted_at: None })
        }
        _ => {
            panic!("Database save failed!");
        }
    }
}

pub fn update_existing_account(id: &i32, name: &str, secret: &str, digits: i32, step: i32, algorithm: &str, db: &Connection) -> Result<Account, rusqlite::Error> {
    let mut update_statement = db.prepare("UPDATE accounts SET name = @name, secret = @secret, totp_step = @step, otp_digits = @digits, totp_algorithm = @algorithm WHERE id = @id")?;
    let mut get_statement = db.prepare("SELECT id, name, secret, totp_step, otp_digits, totp_algorithm FROM accounts WHERE id = @id")?;
    let mut final_algorithm = None::<&str>;

    if algorithm != "" {
        final_algorithm = Some(algorithm);
    }

    update_statement.execute(named_params! { "@id": id, "@name": name, "@secret": secret, "@step": step, "@digits": digits, "@algorithm": final_algorithm })?;
    let mut rows = get_statement.query(named_params! {"@id": id })?;

    match rows.next()? {
        Some(row) => {
            let algorithm = match row.get("totp_algorithm")? {
                Some(string_algorithm) => AccountAlgorithm::string_to_algorithm(string_algorithm),
                None => None
            };

            Ok(Account {id: row.get("id")?, name: row.get("name")?, secret: row.get("secret")?, totp_step: row.get("totp_step")?, otp_digits: row.get("otp_digits")?, algorithm, external_id: None, external_last_updated: None, external_hash: None, deleted_at: None })
        }
        _ => {
            panic!("Account could not be found after update!");
        }
    }
}

pub fn get_all_accounts(db: &Connection, filter: &str) -> Result<Vec<Account>, rusqlite::Error>  {
    let mut statement = db.prepare("SELECT id, name, totp_step, otp_digits, external_id, external_last_updated, external_hash FROM accounts WHERE name LIKE ? AND deleted_at IS NULL ORDER BY name ASC")?;
    let mut rows = statement.query([ "%".to_owned() + filter + "%"])?;
    let mut items = Vec::new();

    while let Some(row) = rows.next()? {
        let title: Account = Account {id: row.get("id")?, name: row.get("name")?, secret: "".to_string(), totp_step: row.get("totp_step")?, otp_digits: row.get("otp_digits")?, algorithm: None, external_id: row.get("external_id")?, external_last_updated: row.get("external_last_updated")?, external_hash: row.get("external_hash")?, deleted_at: None };

        items.push(title);
    }

    Ok(items)
}

pub fn get_account_details_by_id(id: u32, db: &Connection) -> Result<Account, rusqlite::Error> {
    let mut statement = db.prepare("SELECT id, name, secret, totp_step, otp_digits, totp_algorithm, external_id, external_last_updated, external_hash FROM accounts WHERE id = ?")?;
    let mut rows = statement.query([id])?;

    match rows.next()? {
        Some(row) => {
            let algorithm = match row.get("totp_algorithm")? {
                Some(string_algorithm) => AccountAlgorithm::string_to_algorithm(string_algorithm),
                None => None
            };

            Ok(Account {id: row.get("id")?, name: row.get("name")?, secret: row.get("secret")?, totp_step: row.get("totp_step")?, otp_digits: row.get("otp_digits")?, algorithm, external_id: row.get("external_id")?, external_last_updated: row.get("external_last_updated")?, external_hash: row.get("external_hash")?, deleted_at: None})
        }
        _ => {
            Ok(Account {id: 0, name: "".to_string(), secret: "".to_string(), totp_step: 0, otp_digits: 0, algorithm: None, external_id: None, external_last_updated: None, external_hash: None, deleted_at: None })
        }
    }
}

pub fn get_accounts_without_external_id(db: &Connection) -> Result<Vec<Account>, rusqlite::Error>
{
    let mut statement = db.prepare("SELECT id, name, totp_step, otp_digits, external_id, external_last_updated, external_hash FROM accounts WHERE external_id IS NULL ORDER BY name ASC")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();

    while let Some(row) = rows.next()? {
        let title: Account = Account { id: row.get("id")?, name: row.get("name")?, secret: "".to_string(), totp_step: row.get("totp_step")?, otp_digits: row.get("otp_digits")?, algorithm: None, external_id: row.get("external_id")?, external_last_updated: row.get("external_last_updated")?, external_hash: row.get("external_hash")?, deleted_at: None };

        items.push(title);
    }

    Ok(items)
}

pub fn get_account_by_external_id(id: &i32, db: &Connection) -> Result<Option<Account>, rusqlite::Error>
{
    let mut statement = db.prepare("SELECT id, name, secret, totp_step, otp_digits, totp_algorithm, external_id, external_last_updated, external_hash FROM accounts WHERE external_id = ?")?;
    let mut rows = statement.query([id])?;

    match rows.next()? {
        Some(row) => {
            let algorithm = match row.get("totp_algorithm")? {
                Some(string_algorithm) => AccountAlgorithm::string_to_algorithm(string_algorithm),
                None => None
            };

            Ok(Some(Account {id: row.get("id")?, name: row.get("name")?, secret: row.get("secret")?, totp_step: row.get("totp_step")?, otp_digits: row.get("otp_digits")?, algorithm, external_id: row.get("external_id")?, external_last_updated: row.get("external_last_updated")?, external_hash: row.get("external_hash")?, deleted_at: None}))
        }
        _ => {
            Ok(None)
        }
    }
}

pub fn delete_account(account: &Account, db: &Connection) -> Result<bool,  rusqlite::Error> {
    let sync_account = get_main_sync_account(&db).unwrap();

    if sync_account.id != 0 && account.deleted_at.is_none() {
        return soft_delete_account(account, &db)
    }

    return remove_account(account, &db)
}

fn remove_account(account: &Account, db: &Connection) -> Result<bool, rusqlite::Error> {
    let mut statement = db.prepare("DELETE FROM accounts WHERE id = ?")?;
    let affected_rows = statement.execute([account.id])?;

    Ok(affected_rows == 1)
}

fn soft_delete_account(account: &Account, db: &Connection) -> Result<bool, rusqlite::Error> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Error Generating Unix Time");

    let mut statement = db.prepare("UPDATE accounts SET deleted_at = ? WHERE id = ?")?;
    let affected_rows = statement.execute([since_the_epoch.as_secs(), account.id as u64])?;

    Ok(affected_rows == 1)
}

pub fn delete_accounts_without_external_ids(ids: Vec<i32>, db: &Connection) -> Result<usize, rusqlite::Error> {
    let mut statement = db.prepare("DELETE FROM accounts WHERE external_id IS NOT NULL AND external_id NOT IN rarray(?)")?;

    let formatted_ids = Rc::new(ids.iter().copied().map(Value::from).collect::<Vec<Value>>());
    let affected_rows = statement.execute([formatted_ids])?;

    Ok(affected_rows)
}

pub fn account_name_exists(name: &str, db: &Connection) -> Result<bool, rusqlite::Error> {
    let mut statement = db.prepare("SELECT id, name, secret FROM accounts WHERE name = ?")?;
    let mut rows = statement.query([name])?;

    match rows.next()? {
        Some(_row) => {Ok(true)},
        _ => {Ok(false)}
    }
}

pub fn create_sync_account(username: &str, password: &str, url: &str, db: &Connection) -> Result<SyncAccount, rusqlite::Error> {
    let mut statement = db.prepare("INSERT INTO sync_accounts (username, password, url) VALUES (@username, @password, @url)")?;

    statement.execute(named_params! { "@username": username, "@password": encrypt(password), "@url": url })?;

    Ok(get_main_sync_account(db).unwrap())
}

pub fn update_sync_account(sync_account: SyncAccount, db: &Connection) -> Result<bool, rusqlite::Error> {
    let mut statement = db.prepare("UPDATE sync_accounts SET username = @username, password = @password, url = @url FROM sync_accounts WHERE id = @id")?;
    let affected_rows = statement.execute(named_params! {"@id": sync_account.id, "@username": sync_account.username, "@password": sync_account.password, "@url": sync_account.url})?;

    Ok(affected_rows == 1)
}

pub fn delete_sync_account(id: i32, db: &Connection) -> Result<bool, rusqlite::Error> {
    let mut statement = db.prepare("DELETE FROM sync_accounts WHERE id = ?")?;
    let affected_rows = statement.execute([id])?;

    Ok(affected_rows == 1)
}

pub fn get_main_sync_account(db: &Connection) -> Result<SyncAccount, rusqlite::Error> {
    let mut statement = db.prepare("SELECT id, username, password, url FROM sync_accounts LIMIT 1")?;
    let mut rows = statement.query([])?;

    match rows.next()? {
        Some(row) => {
            let encrypted_password: String = row.get("password")?;

            Ok(SyncAccount {id: row.get("id")?, username: row.get("username")?, password: decrypt(&encrypted_password), url: row.get("url")?, token: None })
        }
        _ => {
            Ok(SyncAccount {id: 0, username: "".to_string(), password: "".to_string(), url: "".to_string(), token: None })
        }
    }
}

pub fn set_remote_account(db: &Connection, account: &Account, record: &Record) -> Result<bool, rusqlite::Error> {
    let mut statement = db.prepare("UPDATE accounts SET external_id = @record_id, external_last_updated = @updated, external_hash = @hash WHERE id = @id")?;
    let affected_rows = statement.execute(named_params! {"@record_id": record.id, "@updated": record.updated_at, "@hash": record.sync_hash, "@id": account.id})?;

    Ok(affected_rows == 1)
}

pub fn get_soft_deleted_accounts(db: &Connection) -> Result<Vec<Account>, rusqlite::Error>
{
    let mut statement = db.prepare("SELECT id, name, totp_step, otp_digits, external_id, external_last_updated, external_hash, deleted_at FROM accounts WHERE deleted_at IS NOT NULL")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();

    while let Some(row) = rows.next()? {
        let title: Account = Account {
            id: row.get("id")?,
            name: row.get("name")?,
            secret: "".to_string(),
            totp_step: row.get("totp_step")?,
            otp_digits: row.get("otp_digits")?,
            algorithm: None,
            external_id: row.get("external_id")?,
            external_last_updated: row.get("external_last_updated")?,
            external_hash: row.get("external_hash")?,
            deleted_at: row.get("deleted_at")?
        };

        items.push(title);
    }

    Ok(items)
}

pub fn create_sync_log(db: &Connection, log: String, log_type: SyncLogType) -> Result<SyncLog, rusqlite::Error> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Could not generate UNIX time");
    let timestamp = since_the_epoch.as_secs();
    let final_log_type = SyncLogType::sync_log_to_u16(log_type.clone());

    let mut statement = db.prepare("INSERT INTO sync_logs (log, log_type, timestamp) VALUES (@log, @log_type, @timestamp)")?;
    statement.execute(named_params! { "@log": log, "@log_type": final_log_type, "@timestamp": timestamp})?;

    return Ok(SyncLog {
        id: 0,
        log,
        log_type,
        timestamp
    })
}

pub fn get_sync_logs(db: &Connection) -> Result<Vec<SyncLog>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT id, log, log_type, timestamp FROM sync_logs ORDER BY timestamp DESC LIMIT 10")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();

    while let Some(row) = rows.next()? {
        let log_type = match row.get("log_type")? {
            Some(u16_log_type) => SyncLogType::u16_to_sync_log(u16_log_type).unwrap(),
            None => SyncLogType::ERROR,
        };

        let log: SyncLog = SyncLog {
            id: row.get("id")?,
            log: row.get("log")?,
            log_type,
            timestamp: row.get("timestamp")?,
        };

        items.push(log);
    }

    Ok(items)
}

fn update_database(db: &mut Connection, existing_version: u32) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION {
        m2024_03_31_account_creation::migrate(db, existing_version).expect("FAILED: Account Table Creation - ");
        m2024_04_01_account_timeout_algorithm::migrate(db, existing_version).expect("FAILED: Account timeout algorithm - ");
        m2024_07_01_sync_account_creation::migrate(db, existing_version).expect("FAILED: Sync Account Creation - ");
        m2024_07_15_account_sync_details::migrate(db, existing_version).expect("FAILED: Account External Details - ");
        m2024_09_13_soft_delete_accounts::migrate(db, existing_version).expect("FAILED: Account Soft Delete - ");
        m2024_09_15_remove_sync_error_log::migrate(db,existing_version).expect("FAILED: Sync Error Log - ");
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use libotp::HOTPAlgorithm;
    use rusqlite::Connection;
    use crate::database::{AccountAlgorithm, create_new_account, get_all_accounts, initialize_test_database, SQLITE_TEST_NAME, update_existing_account};
    use crate::database::AccountAlgorithm::{SHA1, SHA512};

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

        let original_account = create_new_account(&name, &secret, &digits, &step, &algorithm, &db).unwrap();
        let result = update_existing_account(&original_account.id, &updated_name, &updated_secret, updated_digits, updated_step, &updated_algorithm,&db);

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

        let original_account = create_new_account(&name, &secret, &digits, &step, &algorithm, &db).unwrap();
        let result = update_existing_account(&original_account.id, &original_account.name, &original_account.secret, original_account.otp_digits, original_account.totp_step, &updated_algorithm,&db);

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
        reset_db(&db);

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

    // fn get_account_details_by_id() {
    //
    // }
    //
    // fn get_account_details_by_id_with_external_details() {
    //
    // }
    //
    // fn set_external_id_to_account() {
    //
    // }
    //
    // fn set_sync_accounts() {
    //
    // }
    //
    // fn update_sync_accounts() {
    //
    // }
    //
    // fn delete_sync_accounts() {
    //
    // }

    fn reset_db(db: &Connection) -> Result<(), rusqlite::Error> {
        let mut statement = db.prepare("DELETE FROM accounts")?;
        statement.execute([])?;

        Ok(())
    }
}