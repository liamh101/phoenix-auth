use rusqlite::Connection;

const MIGRATION_NUMBER: u32 = 3;

pub fn migrate(db: &mut Connection, current_version: u32) -> Result<(), rusqlite::Error> {
    if current_version >= MIGRATION_NUMBER {
        return Ok(());
    }

    db.pragma_update(None, "journal_mode", "WAL")?;

    let tx = db.transaction()?;

    tx.pragma_update(None, "user_version", MIGRATION_NUMBER)?;

    tx.execute_batch(
        "
            CREATE TABLE sync_accounts (
                id integer primary key,
                username VARCHAR(255) NOT NULL,
                password VARCHAR(255) NOT NULL,
                url VARCHAR(2083) NOT NULL
            );",
    )?;

    tx.commit()?;

    Ok(())
}
