use rocket::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        User {
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}
