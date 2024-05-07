use rocket::{fs::FileServer, futures::lock::Mutex};
use rusqlite::Connection;
use crate::db::get_users;

use super::users::User;
use rocket::form::Form;
use std::sync::Arc;
use rocket::State;
use super::db::insert_user;

#[post("/submitUser", data = "<user>")]
async fn post_user(user: Form<User>, conn_mutex: &State<Arc<Mutex<Connection>>>) {

    let user = user.into_inner();   
    insert_user(&*conn_mutex.lock().await, &user).unwrap();
}

#[get("/getUsers")]
async fn get_user(conn_mutex: &State<Arc<Mutex<Connection>>>) {
    get_users(&*conn_mutex.lock().await);
}

#[delete("/deleteUser", data = "<user>")]
async fn delete_user(user: Form<User>) {
    
}

pub async fn launch_rocket(conn: Connection) {
 
    let conn_mutex = Arc::new(Mutex::new(conn));

    rocket::build()
        .manage(conn_mutex)
        .mount("/", FileServer::from("./public"))
        .mount("/", routes![post_user, get_user])
        .launch()
        .await
        .expect("failed to launch rocket server.");

}

