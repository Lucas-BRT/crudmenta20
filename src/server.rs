use crate::db;
use crate::users::User;
use rocket::form::Form;
use rocket::State;
use rocket::{fs::FileServer, futures::lock::Mutex};
use rusqlite::Connection;
use serde_json::{json, Value};
use std::sync::Arc;

#[post("/submitUser", data = "<user>")]
async fn post_user(user: Form<User>, conn_mutex: &State<Arc<Mutex<Connection>>>) {
    let conn = &*conn_mutex.lock().await;
    let user = user.into_inner();

    db::insert_user(conn, &user).unwrap();
}

#[get("/getUsers")]
async fn get_users(conn_mutex: &State<Arc<Mutex<Connection>>>) -> Value {
    json!(db::get_users(&*conn_mutex.lock().await))
}

#[get("/getUser", data = "<user>")]
async fn get_user(user: Form<String>, conn_mutex: &State<Arc<Mutex<Connection>>>) -> Value {
    let conn = &*conn_mutex.lock().await;
    let user = user.into_inner();

    json!(db::get_user(conn, user).unwrap().unwrap())
}

pub async fn launch_rocket(conn: Connection) {
    let conn_mutex = Arc::new(Mutex::new(conn));

    rocket::build()
        .manage(conn_mutex)
        .mount("/", FileServer::from("./public"))
        .mount("/", routes![post_user, get_users, get_user])
        .launch()
        .await
        .expect("failed to launch rocket server.");
}
