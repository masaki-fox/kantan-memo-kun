use rusqlite::{Connection, params,Result};


pub fn save_memo_to_db(memo:&str) -> Result<()>{
    let conn = Connection::open("data/memo.db")?;

    // テーブルがなければ作成（最初だけ）
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL
        )",
        [],
    ).unwrap();

    // INSERT 実行
    conn.execute(
        "INSERT INTO entries (content) VALUES (?1)",
        params![memo],
    )?;
    Ok(())
}