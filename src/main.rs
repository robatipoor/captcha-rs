#[macro_use]
extern crate actix_web;

use actix_web::{http, middleware, web, App, Error, HttpResponse, HttpServer};
use sqlx::prelude::*;
use sqlx::{Pool, SqliteConnection, SqlitePool};

use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web::dev::ServiceRequest;

mod handlers;
mod models;
mod auth;
use handlers::ping;


#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
    let pool = SqlitePool::new("sqlite://test_db.sqlite").await.unwrap();
    HttpServer::new(move || {
        let auth = HttpAuthentication::basic(auth::validator);
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(auth)
            .data(pool.clone())
            .service(ping)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
