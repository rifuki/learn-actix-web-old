/* Streaming Request */
#![allow(non_snake_case)]
use actix_web::{get, web, HttpResponse, Result};
use futures::StreamExt;

#[get("/")]
async fn index(mut payload: web::Payload) -> Result<HttpResponse> {
    let mut bytes = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        println!("chunk: {:?}", &chunk);
        bytes.extend_from_slice(&chunk);
    }

    let bytes_length = bytes.len();
    let response_body = format!("bytes length: {}", bytes_length);
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind(("::", 80))?
        .run()
        .await
}
