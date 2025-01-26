use std::path::PathBuf;
use rusqlite::{Connection, named_params, Rows, Transaction};
use crate::encryption::{encrypt, legacy_decrypt};

const MIGRATION_NUMBER: u32 = 7;

struct LegacyData {
    id: usize,
    encrypted: String,
}

pub fn migrate(db: &mut Connection, current_version: u32, encryption_path: PathBuf) -> Result<(), rusqlite::Error> {
    if current_version >= MIGRATION_NUMBER {
        return Ok(());
    }

    db.pragma_update(None, "journal_mode", "WAL")?;

    let tx = db.transaction()?;

    let accounts = get_accounts(&tx);
    let sync_accounts = get_sync_accounts(&tx);

    tx.pragma_update(None, "user_version", MIGRATION_NUMBER)?;

    for account in accounts.iter() {
        let decrypted_secret = legacy_decrypt(&account.encrypted);

        if decrypted_secret.is_err() {
            continue;
        }

        let encrypted_secret = encrypt(&encryption_path, &decrypted_secret.unwrap()).unwrap();

        tx.execute("UPDATE accounts SET secret = @secret WHERE id = @id", named_params!{"@id": account.id, "@secret": encrypted_secret})?;
    }

    for sync_account in sync_accounts.iter() {
        let decrypted_password = legacy_decrypt(&sync_account.encrypted);

        if decrypted_password.is_err() {
            continue;
        }

        let encrypted_password = encrypt(&encryption_path, &decrypted_password.unwrap()).unwrap();

        tx.execute("UPDATE sync_accounts SET password = @password WHERE id = @id", named_params!{"@id": sync_account.id, "@password": encrypted_password})?;
    }

    tx.commit()?;

    Ok(())
}

fn get_accounts(db: &Transaction) -> Vec<LegacyData> {
    let mut account_statement = db.prepare("SELECT id, secret FROM accounts").unwrap();
    let mut account_rows = account_statement.query({}).unwrap();
    let mut accounts = vec![];

    while let Some(row) = account_rows.next().unwrap() {
        accounts.push(LegacyData {
            id: row.get("id").unwrap(),
            encrypted: row.get("secret").unwrap(),
        });
    }

    accounts
}

fn get_sync_accounts(db: &Transaction) -> Vec<LegacyData> {
    let mut sync_account_statement = db.prepare("SELECT id, password FROM sync_accounts").unwrap();
    let mut sync_account_rows = sync_account_statement.query({}).unwrap();
    let mut sync_accounts = vec![];

    while let Some(row) = sync_account_rows.next().unwrap() {
        sync_accounts.push(LegacyData {
            id: row.get("id").unwrap(),
            encrypted: row.get("password").unwrap(),
        });
    }

    sync_accounts
}