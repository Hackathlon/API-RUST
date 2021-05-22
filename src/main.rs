#[macro_use]
extern crate actix_web;

mod article;
mod user;
mod constants;
mod response;
mod db;

use std::{io, env};
use actix_web::{App, HttpServer, middleware, Responder, web};
use crate::db::DB;
use std::io::Error;
use mongodb::sync::Client;
use mongodb::bson::{Document};
use crate::constants::MONGODB_URL;
use crate::article::Article;

async fn hello_world() -> impl Responder {
    let client = Client::with_uri_str(MONGODB_URL).unwrap();
    let db = client.database("ez-tax");
    let collection = db.collection::<Document>("articles");
    let cursor = collection.find(None, None).unwrap();
    let mut result: Vec<String> = vec![];
    for r in cursor {
        result.push(r.unwrap().to_string());
    }
    //result.iter().take(10).map(|a| a.to_string());
    result.join(",")
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let local = tokio::task::LocalSet::new();
    let sys = actix_web::rt::System::run_in_tokio("server", &local);
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let server_res = HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(article::list)
            .service(article::get)
            .service(article::create)
            .service(article::delete)
            .route("/", web::get().to(hello_world))
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await?;
    sys.await?;
    Ok(server_res)
}