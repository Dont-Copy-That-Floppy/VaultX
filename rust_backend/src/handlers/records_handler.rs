use actix_web::{HttpResponse, Responder};

pub async fn records_handler() -> impl Responder {
    HttpResponse::Ok().json("Returning secure records")
}
