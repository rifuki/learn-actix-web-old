/* Middleware Compresion Post-Data */
#![allow(non_snake_case)]
use actix_web::{get, middleware, web, HttpResponse, Result};
use flate2::{write::GzEncoder, Compression};
use std::io::Write;
use std::vec::Vec;

#[get("/{kotoba}")]
async fn index(path: web::Path<String>) -> Result<HttpResponse> {
    let kotoba = path.into_inner();

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&kotoba.as_bytes())?;
    let compressed_data = encoder.finish()?;

    Ok(HttpResponse::Ok()
        .insert_header(("content-encoding", "gzip"))
        .body(compressed_data))
}

async fn auto() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World! Hello, World!,  World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! World! Hello, World! Hello, World! Hello, World! Hello, World! Hello, World!")
}

fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ap")
            .wrap(middleware::Compress::default())
            .route(web::to(auto)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().configure(config_app).service(index))
        .bind(("::", 80))?
        .run()
        .await
}
