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
    create_users_table(&conn).unwrap();
    
    //let user = User::new("cleitinho", "cleitonzinhosilva@_231.com", "23-0480-1-03409-fdshjk");

    //insert_user(&conn, &user).unwrap();
    
    //get_users(&conn).unwrap();
        
    launch_rocket(conn).await;

}
