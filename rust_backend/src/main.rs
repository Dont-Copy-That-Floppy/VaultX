use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
use env_logger;
use log::info;

mod api;
mod config;
mod db;
mod handlers;
mod models;
mod utils;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();
    info!("Application started");
    
    dotenv().ok();
    let server_address =
        env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    println!("Starting server on {}", server_address);

    HttpServer::new(|| App::new().configure(api::init_routes))
        .bind(server_address)?
        .run()
        .await
}
