use rusqlite::{Connection, Result};
use super::users::User;
use serde_json::{json, Value};

pub fn insert_user(conn: &Connection, user: &User) -> Result<()> {
    let query = format!("INSERT INTO users (name, email, password) VALUES ('{}', '{}', '{}');", user.name, user.email, user.password);
    conn.execute(query.as_str(),()).unwrap();

    Ok(())
}

pub fn get_users(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT * FROM users;").unwrap();
    
    let users = stmt.query_map((), |row| {
        Ok(
            User::new(
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap()
            )
        )
    }).unwrap();
    
    users.for_each(|user| {
        println!("{:#?}", user.unwrap());
    });
    
}

pub fn create_users_table(conn: &Connection) -> Result<()> {
    let query = "CREATE TABLE IF NOT EXISTS users 
        (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT NOT NULL UNIQUE,
        password TEXT NOT NULL
        );";

    conn.execute(query,())?;
    Ok(())
}

pub fn get_connection(database_name: &str) -> Connection {
    let conn = Connection::open(database_name).unwrap();

    conn
}

