use actix_web::web::{Json, Path};
use actix_web::{HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sha2::{Sha256, Digest};
use crate::constants::{APPLICATION_JSON, MONGODB_URL};
use crate::response::Response;
use crate::db::DB;
use mongodb::sync::Client;
use mongodb::bson::*;

pub type Users = Response<User>;

#[derive(Debug, Serialize, Deserialize)]
pub struct User{
    pub id: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub passwd: String,
}

impl User{
    pub fn new(firstname: String, lastname: String, email: String, password: String)-> Self {
        let x = User{
            id: Uuid::new_v4().to_string(),
            firstname: firstname,
            lastname: lastname,
            email: email,
            passwd: password, 
        };
        x
    }
}


// Utility functions
fn is_email_valid(email: &str)-> bool{
    let mut res = false;
    let mut at = false;
    for c in email.chars() {
        if(c == '@'){ at = true;}
        if at{ if c == '.'{ res = true;}}
    }
    res
}




#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest{
    pub firstname: Option<String>, 
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub passwd: Option<String>,
}
impl UserRequest{
    pub fn to_user(&self)-> Option<User>{
        match (&self.firstname, &self.lastname, &self.email, &self.passwd){
            (Some(firstname), Some(lastname), Some(email), Some(password)) => Some(User::new(firstname.to_string(), lastname.to_string(), email.to_string(), password.to_string())),
           (_, _, _, _) => None,
        }
    }
}

#[get("/user/{email}")]
pub async fn get_usr(path: Path<(String, )>)->HttpResponse {
    let client = Client::with_uri_str(MONGODB_URL).unwrap();
    let db = client.database("ez-tax");
    let collection = db.collection::<Document>("users");
    let usr = collection.find_one(doc! {"email": path.0.0}, None).unwrap();
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(usr.unwrap())
}

#[post("/user")]
pub async fn create_usr(user_req: Json<UserRequest>) -> HttpResponse {
    let usr = user_req.to_user().unwrap();
    let client = Client::with_uri_str(MONGODB_URL).unwrap();
    let db = client.database("ez-tax");
    let collection = db.collection::<Document>("users");
    let mut hasher = Sha256::new();
    hasher.update(&usr.passwd);
    let hash: String = format!("{:X}", hasher.finalize());
    let send = collection.insert_one(doc!{
        "firstname": &usr.firstname,
        "lastname": &usr.lastname,
        "email": &usr.email,
        "passwd": hash,
    }, None);
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(usr.id)
        
}