use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub encrypted_data: String,
}
