use crate::models::auth::AuthRequest;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/register")]
pub async fn register(req: web::Json<AuthRequest>) -> impl Responder {
    let register_request = req.into_inner();

    // Use the password field by calculating its length (for demonstration)
    let password_length = register_request.password.len();

    println!(
        "Registering user: {} with device id: {} (password length: {})",
        register_request.username, register_request.device_id, password_length
    );

    // TODO: Store user in DB and handle password hashing securely

    HttpResponse::Ok().json("Registration successful")
}
