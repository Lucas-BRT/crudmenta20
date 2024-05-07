use serde::{Deserialize, Serialize};
use rocket::FromForm;


#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String
}

impl User {
    pub fn new(name: &str, email: &str, password: &str) -> Self {
        User {
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string()
        }
    }
}

