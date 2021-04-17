use std::collections::HashMap;
use std::net::TcpListener;

use actix_web::{App, dev::Server, get, HttpResponse, HttpServer, Responder, web};

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod utils;


async fn index() -> impl Responder {
    let mut response: HashMap<String, String> = HashMap::new();
    response.insert("message".to_string(), "zero to prod app root".to_string());
    HttpResponse::Ok().json(response)
}

