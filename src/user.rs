use actix_web::web::{Json, Path};
use actix_web::{HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::{APPLICATION_JSON};
use crate::response::Response;

pub type Users = Response<User>;

#[derive(Debug, Serialize, Deserialize)]
pub struct User{
    pub id: String,
    pub username: String,
    pub passwd: String,
}

impl User{
    pub fn new(usrname: String, password: String)-> Self {
        let x = User{
            id: Uuid::new_v4().to_string(),
            username: usrname,
            passwd: password, 
        };
        x
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest{
    pub usrname: Option<String>, 
    pub passwd: Option<String>,
}
impl UserRequest{
    pub fn to_user(&self)-> Option<User>{
        match (&self.usrname, &self.passwd){
            (Some(username), Some(password)) => Some(User::new(username.to_string(), password.to_string())),
           (_, _) => None,
        }
    }
}

