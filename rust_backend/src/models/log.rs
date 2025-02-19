use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    timestamp: String,
    event_type: String,
    details: String,
}
