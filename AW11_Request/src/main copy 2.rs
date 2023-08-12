/* Content Encoding */
#![allow(non_snake_case)]
use actix_web::{web, HttpResponse, Result};
use flate2::read::GzDecoder;
use std::io::Read;
use futures::StreamExt;

async fn index(mut body: web::Payload) -> Result<HttpResponse> {
    let mut bytes = web::BytesMut::new();

    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut decoded = String::new();
    decoder.read_to_string(&mut decoded)?;

    Ok(HttpResponse::Ok().body(decoded))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{HttpServer, App};

    HttpServer::new(|| {
        App::new()
        .service(
            web::resource("/")
            .route(web::post().to(index))
        )
    })
    .bind(("::", 80))?
    .run()
    .await
}