mod user;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Request {
    pub authorization: String,
    pub additional: String,
    pub num1: i32,
    pub num2: i32,
    pub data: Vec<user::User>
}