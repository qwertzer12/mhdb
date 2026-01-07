use crate::commands::message::greet;
use rusqlite::{Connection, Result};

#[test]
fn test_greet_external() {
    let result = greet("Universe");
    assert_eq!(result, "Hello, Universe! You've been greeted from Rust!");
}

#[test]
fn init_db_connection() {
    let conn: Result<Connection, rusqlite::Error> = Connection::open_in_memory();
    assert!(conn.is_ok());
}

#[test]
fn db_write() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
                  id    INTEGER PRIMARY KEY,
                  name  TEXT NOT NULL
                  )",
        (),
    )?;
    let rows_affected = conn.execute(
        "INSERT INTO person (name) VALUES (?1)",
        ["Alice"],
    )?;
    assert_eq!(rows_affected, 1);

    Ok(())
}