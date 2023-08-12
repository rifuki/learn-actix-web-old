#![allow(non_snake_case)]
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Homepage")
}

#[get("/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", name))
}

async fn manual_hello(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("HELLO POST {}!", name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home).service(greet).route("/{name}", web::post().to(manual_hello)))
        .bind(("::", 80))
        .unwrap()
        .run()
        .await
}
