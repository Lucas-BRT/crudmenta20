use sqlite::{Connection, Result};

use super::users::User;


pub fn insert_user(conn: &Connection, user: &User) -> Result<()> {
    let query = format!("INSERT INTO users (name, email, password) VALUES ('{}', '{}', '{}');", user.name, user.email, user.password);

    println!("{}", query);

    conn.execute(query).unwrap();

    Ok(())
}

pub fn get_users(conn: &Connection) -> Result<()> {
    let query = format!("SELECT * FROM users");
    
    conn.iterate(query,|users| {
        for &(name, email) in users.iter() {
            println!("{} {:#?}", name, email);
        }
        true
    }).unwrap();

    Ok(())
}

pub fn create_users_table(conn: &Connection) -> Result<()> {
    let query = "CREATE TABLE IF NOT EXISTS users 
        (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT NOT NULL UNIQUE,
        password TEXT NOT NULL
        );";

    conn.execute(query)?;
    Ok(())
}

pub fn get_connection(database_name: &str) -> Connection {
    let conn = Connection::open(database_name).unwrap();

    conn
}

