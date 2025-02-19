use actix_web::{web, HttpResponse, Responder};
use crate::api::authentication::authenticate_user;
use crate::utils::logger::log_auth_attempt;
use mongodb::Client;
use crate::models::auth::AuthRequest;

/// **Authentication handler that utilizes `authenticate_user` from `authentication.rs`.**
pub async fn auth_handler(
    db_client: web::Data<Client>,
    req: web::Json<AuthRequest>,
) -> impl Responder {
    // Basic validation
    if req.username.trim().is_empty() || req.password.trim().is_empty() {
        return HttpResponse::BadRequest().json("Username or password cannot be empty");
    }

    // Authenticate user asynchronously
    match authenticate_user(&db_client, &req.username, &req.password).await {
        Ok(true) => {
            // Log successful attempt
            if let Err(e) = log_auth_attempt(&req.username, true).await {
                eprintln!("Failed to log auth attempt: {}", e);
            }
            HttpResponse::Ok().json("✅ Authentication successful")
        }
        Ok(false) => {
            // Log failed attempt
            if let Err(e) = log_auth_attempt(&req.username, false).await {
                eprintln!("Failed to log auth attempt: {}", e);
            }
            HttpResponse::Unauthorized().json("❌ Invalid credentials")
        }
        Err(e) => {
            eprintln!("Authentication error: {}", e);
            HttpResponse::InternalServerError().json("❌ Internal server error")
        }
    }
}
