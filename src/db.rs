use crate::users::User;
use rusqlite::{params, Connection, Result};

pub fn insert_user(conn: &Connection, user: &User) -> Result<()> {
    let query = format!(
        "INSERT INTO users (name, email, password) VALUES ('{}', '{}', '{}');",
        user.name, user.email, user.password
    );
    conn.execute(query.as_str(), ()).unwrap();

    Ok(())
}

pub fn get_users(conn: &Connection) -> Vec<User> {
    let mut stmt = conn.prepare_cached("SELECT * FROM users;").unwrap();

    let users_iter = stmt
        .query_map((), |row| {
            Ok(User::new(
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
            ))
        })
        .unwrap();

    let mut users: Vec<User> = Vec::new();

    users_iter.for_each(|user| {
        users.push(user.unwrap());
    });

    users
}

pub fn get_user(conn: &Connection, email: String) -> Option<Result<User>> {
    let mut stmt = conn
        .prepare_cached("SELECT * FROM users WHERE email = ?;")
        .unwrap();

    let mut user_iter = stmt
        .query_map(params![email], |row| {
            Ok(User::new(
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
            ))
        })
        .unwrap();

    if let Some(user_result) = user_iter.next() {
        user_result.map(|user| Some(user)).transpose()
    } else {
        None
    }
}

pub fn create_users_table(conn: &Connection) -> Result<()> {
    let query = "CREATE TABLE IF NOT EXISTS users 
        (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT NOT NULL UNIQUE,
        password TEXT NOT NULL
        );";

    conn.execute(query, ())?;
    Ok(())
}

pub fn get_connection(database_name: &str) -> Connection {
    let conn = Connection::open(database_name).unwrap();

    conn
}
