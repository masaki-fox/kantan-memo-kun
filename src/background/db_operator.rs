use rusqlite::{Connection, params,Result};
use std::error::Error;

pub fn start_db_connection(path : &str ) -> Result<Connection, Box<dyn Error>> {
    // conect DB
    let conn = Connection::open(path)?;
    Ok(conn)
}

pub fn save_memo_to_db(memo:&str) -> Result<()>{
    let conn = Connection::open("data/memo.db")?;

    // once create table First time
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL
        )",
        [],
    )?;

    // INSERT 実行
    conn.execute(
        "INSERT INTO entries (content) VALUES (?1)",
        params![memo],
    )?;
    Ok(())
}