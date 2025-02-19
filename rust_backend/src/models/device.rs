use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub device_id: String,
    pub expiration: usize,        // Expiry
    pub user_id: String,
}
