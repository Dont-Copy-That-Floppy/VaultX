use actix_web::{get, post, web, HttpResponse, Responder};
use mongodb::Database;
use crate::db::collections::insert_record;
use crate::models::record::Record;

#[get("/records")]
async fn get_records() -> impl Responder {
    let records = vec![
        Record { id: "1".to_string(), title: "My First Record".to_string(), encrypted_data: "encrypted1".to_string() },
        Record { id: "2".to_string(), title: "Secure Password".to_string(), encrypted_data: "encrypted2".to_string() },
    ];
    HttpResponse::Ok().json(records)
}

#[post("/records")]
async fn create_record(db: web::Data<Database>, record: web::Json<Record>) -> impl Responder {
    let api_record = record.into_inner();

    match insert_record(&db, api_record).await {
        Ok(_) => HttpResponse::Created().json("Record inserted successfully"),
        Err(e) => {
            eprintln!("Error inserting record: {}", e);
            HttpResponse::InternalServerError().body("Failed to insert record")
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_records);
    cfg.service(create_record);
}
