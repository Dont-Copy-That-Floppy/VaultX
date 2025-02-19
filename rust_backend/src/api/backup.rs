use crate::models::encryption::{BackupRequest, RestoreRequest};
use crate::models::record::Record;
use crate::utils::encryption::{decrypt_data, encrypt_data};
use actix_web::{post, web, HttpResponse, Responder};
use futures::TryStreamExt;
use mongodb::{bson, options::FindOptions, Client};

#[post("/backup")]
async fn backup(client: web::Data<Client>, req: web::Json<BackupRequest>) -> impl Responder {
    let db = client.database("valutx");
    let key = req.encryption_key.as_bytes();

    if key.len() != 32 {
        return HttpResponse::BadRequest().body("Encryption key must be exactly 32 bytes");
    }

    let collection = db.collection::<Record>("records");
    let cursor = match collection.find(None, FindOptions::default()).await {
        Ok(cursor) => cursor,
        Err(e) => {
            eprintln!("Failed to fetch records: {}", e);
            return HttpResponse::InternalServerError().body("Backup failed");
        }
    };

    let records: Vec<Record> = match cursor.try_collect().await {
        Ok(records) => records,
        Err(e) => {
            eprintln!("Error collecting records: {}", e);
            return HttpResponse::InternalServerError().body("Failed to collect records");
        }
    };

    let serialized = match bson::to_vec(&records) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Serialization error: {}", e);
            return HttpResponse::InternalServerError().body("Backup failed");
        }
    };

    let encrypted = match encrypt_data(&String::from_utf8_lossy(&serialized), key) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Encryption failed: {}", e);
            return HttpResponse::InternalServerError().body("Backup failed");
        }
    };

    HttpResponse::Ok().body(encrypted)
}

#[post("/restore")]
async fn restore(client: web::Data<Client>, req: web::Json<RestoreRequest>) -> impl Responder {
    let db = client.database("valutx");
    let key = req.encryption_key.as_bytes();

    if key.len() != 32 {
        return HttpResponse::BadRequest().body("Encryption key must be exactly 32 bytes");
    }

    let decrypted = match decrypt_data(&req.backup_data, key) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Decryption failed: {}", e);
            return HttpResponse::InternalServerError().body("Restore failed");
        }
    };

    let records: Vec<Record> = match bson::from_slice(decrypted.as_bytes()) {
        Ok(docs) => docs,
        Err(e) => {
            eprintln!("Deserialization failed: {}", e);
            return HttpResponse::InternalServerError().body("Restore failed");
        }
    };

    let collection = db.collection::<Record>("records");
    if let Err(e) = collection.insert_many(records, None).await {
        eprintln!("Failed to insert records: {}", e);
        return HttpResponse::InternalServerError().body("Restore failed");
    }

    HttpResponse::Ok().body("Database restored successfully")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(backup);
    cfg.service(restore);
}
