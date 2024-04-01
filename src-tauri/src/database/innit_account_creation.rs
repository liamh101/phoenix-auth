use rusqlite::Connection;

const MIGRATION_NUMBER: u32 = 1;

pub fn migrate(db: &mut Connection, current_version: u32) -> Result<(), rusqlite::Error> {
    if current_version >= MIGRATION_NUMBER {
        return Ok(());
    }

    db.pragma_update(None, "journal_mode", "WAL")?;

    let tx = db.transaction()?;

    tx.pragma_update(None, "user_version", MIGRATION_NUMBER)?;

    tx.execute_batch("
            CREATE TABLE accounts (
                id integer primary key,
                name VARCHAR(255) NOT NULL,
                secret VARCHAR(255) NOT NULL
            );"
    )?;

    tx.commit()?;

    Ok(())
}