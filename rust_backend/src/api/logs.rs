use actix_web::{get, web, HttpResponse, Responder};
use mongodb::Client;
use futures::stream::TryStreamExt; // Import for try_collect
use std::env;
use crate::models::log::LogEntry;

#[get("/secure/logs")]
async fn get_logs(client: web::Data<Client>) -> impl Responder {
    // Use environment variable for database name or default to "valutx"
    let db_name = env::var("MONGO_DB_NAME").unwrap_or_else(|_| "valutx".to_string());
    let db = client.database(&db_name);
    let collection = db.collection::<LogEntry>("logs");

    // Fetch logs from the collection
    match collection.find(None, None).await {
        Ok(mut cursor) => {
            let mut logs = Vec::new();

            while let Some(result) = cursor.try_next().await.unwrap_or_else(|_| None) {
                logs.push(result);
            }

            HttpResponse::Ok().json(logs)
        }
        Err(e) => {
            eprintln!("❌ Failed to fetch logs: {}", e);
            HttpResponse::InternalServerError().json("Failed to fetch logs from the database")
        }
    }
}
