use actix_web::web::{Json, Path};
use actix_web::{HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::{APPLICATION_JSON, MONGODB_URL};
use crate::response::Response;
use crate::db::DB;
use mongodb::sync::Client;
use mongodb::bson::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: Option<String>,
    pub passwd: Option<String>,
}

#[post("/login")]
pub async fn login(log_req: Json<LoginRequest>) -> HttpResponse {
    let eml = log_req.email.as_ref();
    let pass = log_req.passwd.as_ref();
    let client = Client::with_uri_str(MONGODB_URL).unwrap();
    let db = client.database("ez-tax");
    let collection = db.collection::<Document>("users");
    let mut found_account: Option<Document> = None;
    let cursor = collection.find_one(doc! {
        "passwd": String::from(pass.unwrap()),
    }, None).unwrap();


    match cursor {
        Some(acc) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(doc! { "result": true }),
        None => HttpResponse::Forbidden()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}