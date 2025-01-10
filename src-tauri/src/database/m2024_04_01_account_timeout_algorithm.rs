use rusqlite::Connection;

const MIGRATION_NUMBER: u32 = 2;

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
                totp_algorithm VARCHAR(100)
            );
            ",
    )?;
    tx.execute_batch(
        "
        INSERT INTO tmp_accounts (id, name, secret, totp_step, otp_digits)
            SELECT id, name, secret, 30, 6 FROM accounts;
    ",
    )?;

    tx.execute_batch("DROP TABLE accounts;")?;
    tx.execute_batch("ALTER TABLE tmp_accounts RENAME TO accounts;")?;

    tx.commit()?;

    Ok(())
}
