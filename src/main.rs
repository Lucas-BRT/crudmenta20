#[macro_use] extern crate rocket;

mod db;
mod server;
mod sheet;
mod users;

use db::{create_users_table, get_connection};
use server::launch_rocket;


#[tokio::main]
async fn main() {
    let conn = get_connection("database.db");
    create_users_table(&conn).expect("failed to create users table");
        
    launch_rocket(conn).await;

}
