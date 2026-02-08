use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}
#[derive(Debug)]
struct Tag {
    id: i32,
    name: String,
}

#[derive(Debug)]
struct PersonTag {
    person_id: i32,
    tag_id: i32,
}

#[derive(Debug)]
struct PersonWithTags {
    id: i32,
    name: String,
    tags: Vec<String>,
}

fn main() -> Result<()> {
    let conn = Connection::open("connection.db")?;

    // person table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (),
    )?;
    
    // tag table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tag (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL UNIQUE
        )",
        (),
    )?;

    // conect person with tag table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person_tag (
            person_id INTEGER,
            tag_id    INTEGER,
            PRIMARY KEY (person_id, tag_id),
            FOREIGN KEY (person_id) REFERENCES person (id),
            FOREIGN KEY (tag_id) REFERENCES tag (id)
        )",
        (),
    )?;

     // Insert sample data
    conn.execute(
        "INSERT OR IGNORE INTO person (name, data) VALUES (?1, ?2)",
        ("Alice", None::<Vec<u8>>),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO person (name, data) VALUES (?1, ?2)",
        ("Bob", None::<Vec<u8>>),
    )?;

    conn.execute("INSERT OR IGNORE INTO tag (name) VALUES (?1)", ("Developer",))?;
    conn.execute("INSERT OR IGNORE INTO tag (name) VALUES (?1)", ("Designer",))?;
    conn.execute("INSERT OR IGNORE INTO tag (name) VALUES (?1)", ("Manager",))?;

    // Associate tags with persons
    conn.execute(
        "INSERT OR IGNORE INTO person_tag (person_id, tag_id) 
         SELECT p.id, t.id FROM person p, tag t 
         WHERE p.name = 'Alice' AND t.name IN ('Developer', 'Designer')",
        (),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO person_tag (person_id, tag_id) 
         SELECT p.id, t.id FROM person p, tag t 
         WHERE p.name = 'Bob' AND t.name IN ('Developer', 'Manager')",
        (),
    )?;

    // Query with JOIN to get persons with their tags
    let mut stmt = conn.prepare(
        "SELECT p.id, p.name, GROUP_CONCAT(t.name, ', ') as tags
         FROM person p
         LEFT JOIN person_tag pt ON p.id = pt.person_id
         LEFT JOIN tag t ON pt.tag_id = t.id
         GROUP BY p.id, p.name
         ORDER BY p.id"
    )?;

    let person_iter = stmt.query_map([], |row| {
        Ok(PersonWithTags {
            id: row.get(0)?,
            name: row.get(1)?,
            tags: row.get::<_, Option<String>>(2)?
                .map(|s| s.split(", ").map(|tag| tag.to_string()).collect())
                .unwrap_or_else(Vec::new),
        })
    })?;

    for person in person_iter {
        println!("Found person with tags: {:?}", person.unwrap());
    }

    Ok(())
}