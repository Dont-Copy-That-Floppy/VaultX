use crate::middleware::auth_middleware::validate_request;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub(crate) mod authentication;
mod backup;
mod devices;
mod logs;
mod records;
mod registration;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let auth_middleware = HttpAuthentication::bearer(validate_request);

    cfg.service(registration::register)
       .service(authentication::login)
       .service(
           web::scope("/secure")
               .wrap(auth_middleware)
               .service(records::get_records)
               .service(devices::register_device)
               .service(logs::get_logs),
       );
}
