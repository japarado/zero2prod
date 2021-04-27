use std::collections::HashMap;

use actix_web::{HttpResponse, Responder};

pub mod configuration;
pub mod domain;
pub mod routes;
pub mod startup;
pub mod utils;
pub mod telemetry;


async fn index() -> impl Responder {
    let mut response: HashMap<String, String> = HashMap::new();
    response.insert("message".to_string(), "zero to prod app root".to_string());
    HttpResponse::Ok().json(response)
}

