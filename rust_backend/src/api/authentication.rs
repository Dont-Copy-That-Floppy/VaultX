use crate::models::auth::{AuthRequest, WebAuthnAuthRequest, WebAuthnVerifyRequest};
use crate::models::user::User;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use log::{error, info};
use mongodb::{bson::doc, Client, Collection};
use serde_json::json;
use url::Url;
use uuid::Uuid;
use webauthn_rs::prelude::{PasskeyRegistration, RegisterPublicKeyCredential};
use webauthn_rs::proto::{AuthenticatorSelectionCriteria, UserVerificationRequirement};
use webauthn_rs::WebauthnBuilder;

/// Extracts the origin dynamically from the incoming request.
fn get_rp_origin(req: &HttpRequest) -> Result<Url, String> {
    if let Some(conn_info) = req.connection_info().host().split_once(':') {
        let scheme = if req.connection_info().scheme() == "https" {
            "https"
        } else {
            "http"
        };
        let origin = format!("{}://{}", scheme, conn_info.0);
        Url::parse(&origin).map_err(|e| format!("Failed to parse URL: {}", e))
    } else {
        Err("Unable to extract host from request".to_string())
    }
}

#[post("/login")]
async fn login(db_client: web::Data<Client>, req: web::Json<AuthRequest>) -> impl Responder {
    match authenticate_user(&db_client, &req.username, &req.password).await {
        Ok(true) => HttpResponse::Ok().json("✅ Login successful"),
        Ok(false) => HttpResponse::Unauthorized().json("❌ Invalid credentials"),
        Err(e) => {
            error!("Login error: {}", e);
            HttpResponse::InternalServerError().json("❌ Internal server error")
        }
    }
}

/// **Asynchronous user authentication function**  
/// Verifies user credentials against the database.
pub async fn authenticate_user(
    db_client: &Client,
    username: &str,
    password: &str,
) -> Result<bool, String> {
    let db = db_client.database("valutx"); // Replace with your DB name
    let users_collection: Collection<User> = db.collection("users");

    // Find user by username
    let filter = doc! { "username": username };
    let user_result = users_collection
        .find_one(filter, None)
        .await
        .map_err(|e| format!("Database query failed: {}", e))?;

    if let Some(user) = user_result {
        // Verify password using Argon2
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| format!("Password hash parsing failed: {}", e))?;

        let argon2 = Argon2::default();
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => {
                info!("User '{}' authenticated successfully.", username);
                Ok(true)
            }
            Err(_) => {
                info!("Invalid credentials for user '{}'.", username);
                Ok(false)
            }
        }
    } else {
        info!("User '{}' not found.", username);
        Ok(false)
    }
}

#[post("/register")]
async fn register_webauthn(
    req: HttpRequest,
    payload: web::Json<WebAuthnAuthRequest>,
) -> impl Responder {
    // Extract rp_origin dynamically from the request
    let rp_origin = match get_rp_origin(&req) {
        Ok(origin) => origin,
        Err(e) => {
            error!("Origin extraction failed: {}", e);
            return HttpResponse::InternalServerError().json("❌ Failed to parse request origin");
        }
    };

    // Initialize WebAuthnBuilder with proper error handling
    let webauthn = match WebauthnBuilder::new("ValutX", &rp_origin)
        .map_err(|e| format!("WebauthnBuilder creation failed: {}", e))
        .and_then(|builder| {
            builder
                .build()
                .map_err(|e| format!("WebAuthn build failed: {}", e))
        }) {
        Ok(webauthn) => webauthn,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().json("❌ WebAuthn initialization failed");
        }
    };

    // Generate a new Uuid for the user handle
    let user_handle_uuid = Uuid::new_v4();

    // Set up authenticator selection criteria with user verification requirement
    let auth_selection = AuthenticatorSelectionCriteria {
        user_verification: UserVerificationRequirement::Preferred,
        ..Default::default()
    };

    // Start passkey registration with proper arguments
    let (ccr, _registration) = match webauthn.start_passkey_registration(
        user_handle_uuid,
        &payload.user_id,
        &auth_selection,
        None, // No excluded credentials
    ) {
        Ok(result) => result,
        Err(e) => {
            error!("Passkey registration failed: {}", e);
            return HttpResponse::InternalServerError()
                .json("❌ Failed to start passkey registration");
        }
    };

    // Return the challenge from CreationChallengeResponse
    HttpResponse::Ok().json(json!({ "challenge": ccr.challenge }))
}

#[post("/verify")]
async fn verify_webauthn(
    http_req: HttpRequest,
    req: web::Json<WebAuthnVerifyRequest>,
) -> impl Responder {
    // Extract rp_origin dynamically from the request
    let rp_origin = match get_rp_origin(&http_req) {
        Ok(origin) => origin,
        Err(e) => {
            error!("Origin extraction failed: {}", e);
            return HttpResponse::InternalServerError().json("❌ Failed to parse request origin");
        }
    };

    // Initialize WebAuthnBuilder with proper error handling
    let webauthn = match WebauthnBuilder::new("ValutX", &rp_origin)
        .map_err(|e| format!("WebauthnBuilder creation failed: {}", e))
        .and_then(|builder| {
            builder
                .build()
                .map_err(|e| format!("WebAuthn build failed: {}", e))
        }) {
        Ok(webauthn) => webauthn,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().json("❌ WebAuthn initialization failed");
        }
    };

    // Convert req.credential_id and req.signature into expected types
    let register_public_key_credential =
        match serde_json::from_str::<RegisterPublicKeyCredential>(&req.credential_id) {
            Ok(cred) => cred,
            Err(e) => {
                error!("Failed to parse RegisterPublicKeyCredential: {}", e);
                return HttpResponse::BadRequest().json("❌ Invalid credential data");
            }
        };

    #[derive(serde::Deserialize)]
    let passkey_registration = match serde_json::from_str::<PasskeyRegistration>(&req.signature) {
        Ok(sig) => sig,
        Err(e) => {
            error!("Failed to parse PasskeyRegistration: {}", e);
            return HttpResponse::BadRequest().json("❌ Invalid signature data");
        }
    };

    // Finish passkey registration with the correct arguments
    let result = webauthn.finish_passkey_registration(
        &register_public_key_credential, // Correct type
        &passkey_registration,           // Correct type
    );

    match result {
        Ok(_) => HttpResponse::Ok().json("✅ Biometric authentication successful"),
        Err(e) => {
            error!("Biometric verification failed: {}", e);
            HttpResponse::Unauthorized().json("❌ Failed biometric authentication")
        }
    }
}
