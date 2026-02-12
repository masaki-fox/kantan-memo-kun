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
        "CREATE TABLE IF NOT EXISTS memos (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL
        )",
        [],
    )?;

    // INSERT 実行
    conn.execute(
        "INSERT INTO memos (content) VALUES (?1)",
        params![memo],
    )?;
    Ok(())
}

pub fn get_all_memos(conn: &Connection) -> Result<Vec<(i32, String)>> {
    let mut stmt = conn.prepare("SELECT id, content FROM memos")?;
    let rows =
        stmt.query_map([], |row| {Ok((row.get(0)?, row.get(1)?))}
    )?;

    let mut items = Vec::new();
    for item in rows {
        items.push(item?);
    }
    Ok(items)
}