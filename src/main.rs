#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::net::{IpAddr, Ipv4Addr};

mod db;
mod jobs;
mod error_handler;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();
    env_logger::init();

    const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    
    let host = env::var("HOST")
        .ok()
        .and_then(|host| host.parse().ok())
        .unwrap_or(LOCALHOST);

    let port = env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080);

    HttpServer::new(|| App::new().configure(jobs::init_routes).wrap(
       Cors::new()
        .allowed_origin(env::var("CORS").unwrap().as_str())
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600)
        .finish()
    )
    .wrap(Logger::default()))
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
