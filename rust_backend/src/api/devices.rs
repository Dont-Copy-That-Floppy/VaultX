use actix_web::{post, web, HttpResponse, Responder};
use mongodb::{bson::doc, Client, Collection};
use crate::models::device::Device;

#[post("/devices/preapprove")]
pub async fn preapprove_device(
    client: web::Data<Client>,
    user: web::ReqData<String>,
) -> impl Responder {
    let db = client.database("valutx");
    let user_id = user.into_inner().clone();
    let collection: Collection<mongodb::bson::Document> = db.collection("devices");

    match collection
        .update_one(
            doc! { "user_id": user_id },
            doc! { "$set": { "new_device_flag": true } },
            None,
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Device approval flag enabled"),
        Err(e) => {
            eprintln!("Failed to update device approval flag: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to update device approval flag")
        }
    }
}

#[post("/devices/approve")]
pub async fn approve_device(
    client: web::Data<Client>,
    body: web::Json<Device>,
    user: web::ReqData<String>,
) -> impl Responder {
    let db = client.database("valutx");
    let user_id = user.into_inner();
    let collection: Collection<mongodb::bson::Document> = db.collection("devices");

    // Look for an approval flag for the device for the given user.
    match collection
        .find_one(doc! { "user_id": &user_id, "new_device_flag": true }, None)
        .await
    {
        Ok(Some(_)) => {
            // Insert approved device document.
            match collection
                .insert_one(
                    doc! {
                        "user_id": &user_id,
                        "device_id": &body.device_id,
                        "approved": true
                    },
                    None,
                )
                .await
            {
                Ok(_) => HttpResponse::Ok().json("Device approved successfully"),
                Err(e) => {
                    eprintln!("Failed to approve device: {:?}", e);
                    HttpResponse::InternalServerError().json("Failed to approve device")
                }
            }
        }
        Ok(None) => HttpResponse::Unauthorized().json("Device approval not enabled"),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json("Database error")
        }
    }
}

#[post("/devices/register")]
pub async fn register_device(req: web::Json<Device>) -> impl Responder {
    // Now the fields are used, so the warning will disappear.
    println!("Registering device '{}' for user '{}'", req.device_id, req.user_id);
    HttpResponse::Ok().json("Device registered successfully")
}

