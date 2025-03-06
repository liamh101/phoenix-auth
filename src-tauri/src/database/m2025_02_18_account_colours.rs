use rusqlite::Connection;

const MIGRATION_NUMBER: u32 = 9;

pub fn migrate(db: &mut Connection, current_version: u32) -> Result<(), rusqlite::Error> {
    if current_version >= MIGRATION_NUMBER {
        return Ok(());
    }

    db.pragma_update(None, "journal_mode", "WAL")?;

    let tx = db.transaction()?;

    tx.pragma_update(None, "user_version", MIGRATION_NUMBER)?;

    tx.execute_batch(
        "
            CREATE TABLE tmp_accounts (
                id INTEGER primary key,
                name VARCHAR(255) NOT NULL,
                secret VARCHAR(255) NOT NULL,
                totp_step INTEGER NOT NULL,
                otp_digits INTEGER NOT NULL,
                colour VARCHAR(6) NOT NULL,
                totp_algorithm VARCHAR(100),
                external_id INTEGER,
                external_last_updated INTEGER,
                external_hash VARCHAR(128),
                deleted_at INTEGER
            );
            ",
    )?;
    tx.execute_batch("
        INSERT INTO tmp_accounts (id, name, secret, totp_step, otp_digits, colour, totp_algorithm, external_id, external_last_updated, external_hash, deleted_at)
            SELECT id, name, secret, totp_step, otp_digits, '5c636a', totp_algorithm, external_id, external_last_updated, external_hash, deleted_at FROM accounts;
    ")?;

    tx.execute_batch("DROP TABLE accounts;")?;
    tx.execute_batch("ALTER TABLE tmp_accounts RENAME TO accounts;")?;

    tx.commit()?;

    Ok(())
}
