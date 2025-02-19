use chrono::Utc;
use log::info;
use mongodb::{bson::doc, Client};

pub async fn log_event(client: &Client, user_id: &str, event_type: &str, details: &str) {
    let db = client.database("valutx");
    let collection = db.collection("logs");

    collection
        .insert_one(
            doc! {
                "user_id": user_id,
                "timestamp": Utc::now().to_string(),
                "event_type": event_type,
                "details": details
            },
            None,
        )
        .await
        .unwrap();
}

/// Asynchronous logging function
pub async fn log_auth_attempt(username: &str, success: bool) -> Result<(), String> {
    // Example async logging (e.g., writing to DB or file)
    let status = if success { "SUCCESS" } else { "FAILURE" };
    info!("User '{}' attempted to log in: {}", username, status);

    // Simulate async operation if needed (remove if unnecessary)
    // tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    Ok(())
}
