use rusqlite::Connection;

const MIGRATION_NUMBER: u32 = 6;

pub fn migrate(db: &mut Connection, current_version: u32) -> Result<(), rusqlite::Error> {
    if current_version >= MIGRATION_NUMBER {
        return Ok(());
    }

    db.pragma_update(None, "journal_mode", "WAL")?;

    let tx = db.transaction()?;

    tx.pragma_update(None, "user_version", MIGRATION_NUMBER)?;

    tx.execute_batch(
        "
            CREATE TABLE sync_logs (
                id INTEGER primary key,
                log VARCHAR(2083) NOT NULL,
                log_type INTEGER NOT NULL,
                timestamp INTEGER NOT NULL
            );",
    )?;

    tx.commit()?;

    Ok(())
}
