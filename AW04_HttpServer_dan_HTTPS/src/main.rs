#![allow(non_snake_case)]

use actix_web::{HttpServer, App, Responder, HttpResponse, web};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("index\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(index))
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}
