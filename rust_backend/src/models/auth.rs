use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
    pub device_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebAuthnAuthRequest {
    pub user_id: String,
    pub challenge: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebAuthnVerifyRequest {
    pub user_id: String,
    pub credential_id: String,
    pub passkey_session_id: String,
}
