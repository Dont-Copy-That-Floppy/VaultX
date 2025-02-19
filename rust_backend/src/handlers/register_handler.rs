use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::utils::hashing::hash_password;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String,
    device_id: String,
}

pub async fn register_handler(req: web::Json<AuthRequest>) -> impl Responder {
    let _hashed_password = hash_password(&req.password);
    println!("Storing user: {} with device {}", req.username, req.device_id);
    HttpResponse::Ok().json("User registered successfully")
}
