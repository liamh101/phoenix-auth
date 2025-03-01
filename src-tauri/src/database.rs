use crate::sync_api::Record;
use libotp::HOTPAlgorithm;
use rusqlite::types::Value;
use rusqlite::{named_params, Connection};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

const SQLITE_NAME: &str = "Phoenix.sqlite";
const CURRENT_DB_VERSION: u32 = 9;

mod m2024_03_31_account_creation;
mod m2024_04_01_account_timeout_algorithm;
mod m2024_07_01_sync_account_creation;
mod m2024_07_15_account_sync_details;
mod m2024_09_13_soft_delete_accounts;
mod m2024_09_15_remove_sync_error_log;

#[cfg(test)]
#[path = "./database_test.rs"]
mod tests;
mod m2025_01_22_migrate_encryption;
mod m2025_02_08_settings;
mod m2025_02_18_account_colours;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub secret: String,
    pub totp_step: i32,
    pub otp_digits: i32,
    pub colour: String,
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Theme {
    DEFAULT,
    DARK,
    LIGHT,
}

impl Theme {
    pub fn num_to_theme(num: i8) -> Theme {
        match num {
            1 => Theme::DARK,
            2 => Theme::LIGHT,
            _ => Theme::DEFAULT
        }
    }

    pub fn theme_to_num(&self) -> i8 {
        match *self {
            Theme::DEFAULT => 0,
            Theme::DARK => 1,
            Theme::LIGHT => 2,
        }
    }
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
            _ => None,
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum SyncLogType {
    ERROR = 1,
}

impl SyncLogType {
    pub fn u16_to_sync_log(sync_log: u16) -> Option<SyncLogType> {
        match sync_log {
            1 => Option::from(SyncLogType::ERROR),
            _ => None,
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

#[derive(Serialize, Deserialize)]
pub struct Setting {
    pub id: i32,
    pub theme: Theme,
}

pub fn initialize_prod_database(database_path: PathBuf, encryption_path: PathBuf) -> Result<Connection, rusqlite::Error> {
    fs::create_dir_all(&database_path).expect("The app data directory should be created.");
    let sqlite_path = database_path.join(SQLITE_NAME);

    initialize_database(sqlite_path, encryption_path)
}

fn initialize_database(database_location: PathBuf, encryption_path: PathBuf) -> Result<Connection, rusqlite::Error> {
    let mut db = Connection::open(database_location)?;
    rusqlite::vtab::array::load_module(&db)?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| row.get(0))?;
    drop(user_pragma);

    let _ = update_database(&mut db, existing_user_version, encryption_path);

    Ok(db)
}

pub fn create_new_account(
    name: &str,
    secret: &str,
    digits: &i32,
    step: &i32,
    colour: &str,
    algorithm: &str,
    db: &Connection,
) -> Result<Account, rusqlite::Error> {
    let mut insert_statement = db.prepare("INSERT INTO accounts (name, secret, totp_step, otp_digits, colour, totp_algorithm) VALUES (@name, @secret, @step, @digits, @colour, @algorithm)")?;
    let mut get_statement = db.prepare("SELECT id, name, secret, totp_step, otp_digits, colour, totp_algorithm FROM accounts WHERE name = @name AND secret = @secret")?;
    let mut final_algorithm = None::<&str>;

    if !algorithm.is_empty() {
        final_algorithm = Some(algorithm);
    }

    insert_statement.execute(named_params! { "@name": name, "@secret": secret, "@step": step, "@digits": digits, "@colour": colour, "@algorithm": final_algorithm })?;
    let mut rows = get_statement.query(named_params! {"@name": name, "@secret": secret})?;

    match rows.next()? {
        Some(row) => {
            let algorithm = match row.get("totp_algorithm")? {
                Some(string_algorithm) => AccountAlgorithm::string_to_algorithm(string_algorithm),
                None => None,
            };

            Ok(Account {
                id: row.get("id")?,
                name: row.get("name")?,
                secret: row.get("secret")?,
                totp_step: row.get("totp_step")?,
                otp_digits: row.get("otp_digits")?,
                colour: row.get("colour")?,
                algorithm,
                external_id: None,
                external_last_updated: None,
                external_hash: None,
                deleted_at: None,
            })
        }
        _ => {
            panic!("Database save failed!");
        }
    }
}

pub fn update_existing_account(
    id: &i32,
    name: &str,
    secret: &str,
    digits: i32,
    step: i32,
    colour: &str,
    algorithm: &str,
    db: &Connection,
) -> Result<Account, rusqlite::Error> {
    let mut update_statement = db.prepare("UPDATE accounts SET name = @name, secret = @secret, totp_step = @step, otp_digits = @digits, colour = @colour, totp_algorithm = @algorithm WHERE id = @id")?;
    let mut get_statement = db.prepare("SELECT id, name, secret, totp_step, otp_digits, colour, totp_algorithm FROM accounts WHERE id = @id")?;
    let mut final_algorithm = None::<&str>;

    if !algorithm.is_empty() {
        final_algorithm = Some(algorithm);
    }

    update_statement.execute(named_params! { "@id": id, "@name": name, "@secret": secret, "@step": step, "@digits": digits, "@colour": colour, "@algorithm": final_algorithm })?;
    let mut rows = get_statement.query(named_params! {"@id": id })?;

    match rows.next()? {
        Some(row) => {
            let algorithm = match row.get("totp_algorithm")? {
                Some(string_algorithm) => AccountAlgorithm::string_to_algorithm(string_algorithm),
                None => None,
            };

            Ok(Account {
                id: row.get("id")?,
                name: row.get("name")?,
                secret: row.get("secret")?,
                totp_step: row.get("totp_step")?,
                otp_digits: row.get("otp_digits")?,
                colour: row.get("colour")?,
                algorithm,
                external_id: None,
                external_last_updated: None,
                external_hash: None,
                deleted_at: None,
            })
        }
        _ => {
            panic!("Account could not be found after update!");
        }
    }
}

pub fn get_all_accounts(db: &Connection, filter: &str) -> Result<Vec<Account>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT id, name, totp_step, otp_digits, colour, external_id, external_last_updated, external_hash FROM accounts WHERE name LIKE ? AND deleted_at IS NULL ORDER BY name ASC")?;
    let mut rows = statement.query(["%".to_owned() + filter + "%"])?;
    let mut items = Vec::new();

    while let Some(row) = rows.next()? {
        let title: Account = Account {
            id: row.get("id")?,
            name: row.get("name")?,
            secret: "".to_string(),
            totp_step: row.get("totp_step")?,
            otp_digits: row.get("otp_digits")?,
            colour: row.get("colour")?,
            algorithm: None,
            external_id: row.get("external_id")?,
            external_last_updated: row.get("external_last_updated")?,
            external_hash: row.get("external_hash")?,
            deleted_at: None,
        };

        items.push(title);
    }

    Ok(items)
}

pub fn get_account_details_by_id(id: u32, db: &Connection) -> Result<Account, rusqlite::Error> {
    let mut statement = db.prepare("SELECT id, name, secret, totp_step, otp_digits, colour, totp_algorithm, external_id, external_last_updated, external_hash FROM accounts WHERE id = ?")?;
    let mut rows = statement.query([id])?;

    match rows.next()? {
        Some(row) => {
            let algorithm = match row.get("totp_algorithm")? {
                Some(string_algorithm) => AccountAlgorithm::string_to_algorithm(string_algorithm),
                None => None,
            };

            Ok(Account {
                id: row.get("id")?,
                name: row.get("name")?,
                secret: row.get("secret")?,
                totp_step: row.get("totp_step")?,
                otp_digits: row.get("otp_digits")?,
                colour: row.get("colour")?,
                algorithm,
                external_id: row.get("external_id")?,
                external_last_updated: row.get("external_last_updated")?,
                external_hash: row.get("external_hash")?,
                deleted_at: None,
            })
        }
        _ => Ok(Account {
            id: 0,
            name: "".to_string(),
            secret: "".to_string(),
            totp_step: 0,
            otp_digits: 0,
            colour: "".to_string(),
            algorithm: None,
            external_id: None,
            external_last_updated: None,
            external_hash: None,
            deleted_at: None,
        }),
    }
}

pub fn get_accounts_without_external_id(db: &Connection) -> Result<Vec<Account>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT id, name, totp_step, otp_digits, colour, external_id, external_last_updated, external_hash FROM accounts WHERE external_id IS NULL ORDER BY name ASC")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();

    while let Some(row) = rows.next()? {
        let title: Account = Account {
            id: row.get("id")?,
            name: row.get("name")?,
            secret: "".to_string(),
            totp_step: row.get("totp_step")?,
            otp_digits: row.get("otp_digits")?,
            colour: row.get("colour")?,
            algorithm: None,
            external_id: row.get("external_id")?,
            external_last_updated: row.get("external_last_updated")?,
            external_hash: row.get("external_hash")?,
            deleted_at: None,
        };

        items.push(title);
    }

    Ok(items)
}

pub fn get_account_by_external_id(
    id: &i32,
    db: &Connection,
) -> Result<Option<Account>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT id, name, secret, totp_step, otp_digits, colour, totp_algorithm, external_id, external_last_updated, external_hash FROM accounts WHERE external_id = ?")?;
    let mut rows = statement.query([id])?;

    match rows.next()? {
        Some(row) => {
            let algorithm = match row.get("totp_algorithm")? {
                Some(string_algorithm) => AccountAlgorithm::string_to_algorithm(string_algorithm),
                None => None,
            };

            Ok(Some(Account {
                id: row.get("id")?,
                name: row.get("name")?,
                secret: row.get("secret")?,
                totp_step: row.get("totp_step")?,
                otp_digits: row.get("otp_digits")?,
                colour: row.get("colour")?,
                algorithm,
                external_id: row.get("external_id")?,
                external_last_updated: row.get("external_last_updated")?,
                external_hash: row.get("external_hash")?,
                deleted_at: None,
            }))
        }
        _ => Ok(None),
    }
}

pub fn delete_account(account: &Account, db: &Connection) -> Result<bool, rusqlite::Error> {
    let sync_account = get_main_sync_account(db).unwrap();

    if sync_account.id != 0 && account.deleted_at.is_none() {
        return soft_delete_account(account, db);
    }

    remove_account(account, db)
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

pub fn delete_accounts_without_external_ids(
    ids: Vec<i32>,
    db: &Connection,
) -> Result<usize, rusqlite::Error> {
    let mut statement = db.prepare(
        "DELETE FROM accounts WHERE external_id IS NOT NULL AND external_id NOT IN rarray(?)",
    )?;

    let formatted_ids = Rc::new(ids.iter().copied().map(Value::from).collect::<Vec<Value>>());
    let affected_rows = statement.execute([formatted_ids])?;

    Ok(affected_rows)
}

pub fn account_name_exists(name: &str, db: &Connection) -> Result<bool, rusqlite::Error> {
    let mut statement = db.prepare("SELECT id, name, secret FROM accounts WHERE name = ? AND deleted_at IS NULL")?;
    let mut rows = statement.query([name])?;

    match rows.next()? {
        Some(_row) => Ok(true),
        _ => Ok(false),
    }
}

pub fn create_sync_account(
    username: &str,
    password: &str,
    url: &str,
    db: &Connection,
) -> Result<SyncAccount, rusqlite::Error> {
    let mut statement = db.prepare(
        "INSERT INTO sync_accounts (username, password, url) VALUES (@username, @password, @url)",
    )?;

    statement.execute(
        named_params! { "@username": username, "@password": password, "@url": url },
    )?;

    Ok(get_main_sync_account(db).unwrap())
}

pub fn update_sync_account(
    sync_account: SyncAccount,
    db: &Connection,
) -> Result<bool, rusqlite::Error> {
    let mut statement = db.prepare("UPDATE sync_accounts SET username = @username, password = @password, url = @url WHERE id = @id")?;
    let affected_rows = statement.execute(named_params! {"@id": sync_account.id, "@username": sync_account.username, "@password": sync_account.password, "@url": sync_account.url})?;

    Ok(affected_rows == 1)
}

pub fn delete_sync_account(id: i32, db: &Connection) -> Result<bool, rusqlite::Error> {
    let mut statement = db.prepare("DELETE FROM sync_accounts WHERE id = ?")?;
    let affected_rows = statement.execute([id])?;

    Ok(affected_rows == 1)
}

pub fn get_main_sync_account(db: &Connection) -> Result<SyncAccount, rusqlite::Error> {
    let mut statement =
        db.prepare("SELECT id, username, password, url FROM sync_accounts LIMIT 1")?;
    let mut rows = statement.query([])?;

    match rows.next()? {
        Some(row) => {
            Ok(SyncAccount {
                id: row.get("id")?,
                username: row.get("username")?,
                password: row.get("password")?,
                url: row.get("url")?,
                token: None,
            })
        }
        _ => Ok(SyncAccount {
            id: 0,
            username: "".to_string(),
            password: "".to_string(),
            url: "".to_string(),
            token: None,
        }),
    }
}

pub fn set_remote_account(
    db: &Connection,
    account: &Account,
    record: &Record,
) -> Result<bool, rusqlite::Error> {
    let mut statement = db.prepare("UPDATE accounts SET external_id = @record_id, external_last_updated = @updated, external_hash = @hash WHERE id = @id")?;
    let affected_rows = statement.execute(named_params! {"@record_id": record.id, "@updated": record.updated_at, "@hash": record.sync_hash, "@id": account.id})?;

    Ok(affected_rows == 1)
}

pub fn update_local_updated_at(
    db: &Connection,
    account: &Account,
) -> Result<bool, rusqlite::Error> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Could not generate UNIX time");
    let timestamp = since_the_epoch.as_secs();

    let mut statement =
        db.prepare("UPDATE accounts SET external_last_updated = @updated WHERE id = @id")?;
    let affected_rows =
        statement.execute(named_params! {"@updated": timestamp, "@id": account.id})?;

    Ok(affected_rows == 1)
}

pub fn get_soft_deleted_accounts(db: &Connection) -> Result<Vec<Account>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT id, name, totp_step, otp_digits, colour, external_id, external_last_updated, external_hash, deleted_at FROM accounts WHERE deleted_at IS NOT NULL")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();

    while let Some(row) = rows.next()? {
        let title: Account = Account {
            id: row.get("id")?,
            name: row.get("name")?,
            secret: "".to_string(),
            totp_step: row.get("totp_step")?,
            otp_digits: row.get("otp_digits")?,
            colour: row.get("colour")?,
            algorithm: None,
            external_id: row.get("external_id")?,
            external_last_updated: row.get("external_last_updated")?,
            external_hash: row.get("external_hash")?,
            deleted_at: row.get("deleted_at")?,
        };

        items.push(title);
    }

    Ok(items)
}

pub fn create_sync_log(
    db: &Connection,
    log: String,
    log_type: SyncLogType,
) -> Result<SyncLog, rusqlite::Error> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Could not generate UNIX time");
    let timestamp = since_the_epoch.as_secs();
    let final_log_type = SyncLogType::sync_log_to_u16(log_type.clone());

    let mut statement = db.prepare(
        "INSERT INTO sync_logs (log, log_type, timestamp) VALUES (@log, @log_type, @timestamp)",
    )?;
    statement.execute(
        named_params! { "@log": log, "@log_type": final_log_type, "@timestamp": timestamp},
    )?;

    Ok(SyncLog {
        id: 0,
        log,
        log_type,
        timestamp,
    })
}

pub fn get_sync_logs(db: &Connection) -> Result<Vec<SyncLog>, rusqlite::Error> {
    let mut statement = db.prepare(
        "SELECT id, log, log_type, timestamp FROM sync_logs ORDER BY timestamp DESC LIMIT 10",
    )?;
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

pub fn get_settings(db: &Connection) -> Result<Setting, rusqlite::Error> {
    let mut statement = db.prepare(
        "SELECT id, theme FROM settings ORDER BY id DESC LIMIT 1",
    )?;

    let mut rows = statement.query([])?;
    match rows.next()? {
        Some(row) => {
            Ok(Setting {
                id: row.get("id")?,
                theme: Theme::num_to_theme(row.get("theme")?),
            })
        }
        _ => Ok(Setting {
            id: 0,
            theme: Theme::DEFAULT,
        }),
    }
}

pub fn save_settings(db: &Connection, theme: Theme) -> Result<Setting, rusqlite::Error> {
    let settings = get_settings(db).unwrap();

    if settings.id != 0 {
        return update_settings(db, settings.id, theme);
    }

    return create_settings(db, theme);
}

fn create_settings(db: &Connection, theme: Theme) -> Result<Setting, rusqlite::Error> {
    let mut statement = db.prepare(
        "INSERT INTO settings (theme) VALUES (@theme)",
    )?;
    statement.execute(
        named_params! { "@theme": theme.theme_to_num()},
    )?;

    Ok(get_settings(db).unwrap())
}

fn update_settings(db: &Connection, id: i32, theme: Theme) -> Result<Setting, rusqlite::Error> {
    let mut statement = db.prepare(
        "UPDATE settings SET theme = @theme WHERE id = @id"
    )?;
    statement.execute(
        named_params! { "@id": id, "@theme": theme.theme_to_num()}
    )?;

    Ok(get_settings(db).unwrap())
}

fn update_database(db: &mut Connection, existing_version: u32, encryption_path: PathBuf) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION {
        m2024_03_31_account_creation::migrate(db, existing_version)
            .expect("FAILED: Account Table Creation - ");
        m2024_04_01_account_timeout_algorithm::migrate(db, existing_version)
            .expect("FAILED: Account timeout algorithm - ");
        m2024_07_01_sync_account_creation::migrate(db, existing_version)
            .expect("FAILED: Sync Account Creation - ");
        m2024_07_15_account_sync_details::migrate(db, existing_version)
            .expect("FAILED: Account External Details - ");
        m2024_09_13_soft_delete_accounts::migrate(db, existing_version)
            .expect("FAILED: Account Soft Delete - ");
        m2024_09_15_remove_sync_error_log::migrate(db, existing_version)
            .expect("FAILED: Sync Error Log - ");
        m2025_01_22_migrate_encryption::migrate(db, existing_version, encryption_path)
            .expect("FAILED: Migrate Encryption - ");
        m2025_02_08_settings::migrate(db, existing_version)
            .expect("FAILED: Settings - ");
        m2025_02_18_account_colours::migrate(db, existing_version)
            .expect("Failed: Account Colours - ")
    }

    Ok(())
}
