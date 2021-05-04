use std::collections::HashMap;

use actix_web::{HttpResponse, Responder};

pub mod configuration;
pub mod domain;
pub mod routes;
pub mod startup;
pub mod telemetry;
pub mod utils;
pub mod email_client;

async fn index() -> impl Responder {
    let mut response: HashMap<String, String> = HashMap::new();
    response.insert("message".to_string(), "zero to prod app root".to_string());
    HttpResponse::Ok().json(response)
}
