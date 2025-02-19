use crate::db::mongo_client::get_database;
use crate::models::device::Device;
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use mongodb::{bson::doc, Collection};
use std::env;

pub async fn validate_request(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    let jwt_secret = match env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => {
            return Err((
                actix_web::error::ErrorInternalServerError("JWT_SECRET not set"),
                req,
            ));
        }
    };

    // Decode JWT token
    let decoded_token = decode::<Device>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    );

    let token_data = match decoded_token {
        Ok(data) => data,
        Err(_) => {
            return Err((actix_web::error::ErrorUnauthorized("Invalid token"), req));
        }
    };

    // Connect to MongoDB
    let mongo_uri = match env::var("MONGO_URI") {
        Ok(uri) => uri,
        Err(_) => {
            return Err((
                actix_web::error::ErrorInternalServerError("MONGO_URI not set"),
                req,
            ));
        }
    };

    let db = get_database(&mongo_uri).await;
    let users_collection: Collection<mongodb::bson::Document> = db.collection("users");

    // Validate the user from the database
    let user_result = users_collection
        .find_one(
            doc! { "user_id": &token_data.claims.user_id, "device_id": &token_data.claims.device_id },
            None,
        )
        .await;

    match user_result {
        Ok(Some(_user)) => {
            req.extensions_mut().insert(token_data.claims);
            Ok(req)
        }
        Ok(None) => Err((
            actix_web::error::ErrorUnauthorized("Unauthorized device"),
            req,
        )),
        Err(_) => Err((
            actix_web::error::ErrorInternalServerError("Database error"),
            req,
        )),
    }
}
