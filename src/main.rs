#[macro_use]
extern crate actix_web;

mod article;
mod constants;
mod response;

use std::{io, env};
use actix_web::{App, HttpServer, middleware};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(article::list)
            .service(article::get)
            .service(article::create)
            .service(article::delete)
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}