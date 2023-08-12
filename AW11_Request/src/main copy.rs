/* Load Incoming Payload */
#![allow(non_snake_case)]
use actix_web::{post, web, HttpResponse, Error, error};
use serde::{Deserialize, Serialize};
use futures::StreamExt;

#[derive(Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8
}

const MAX_SIZE: usize = 256_144;

#[post("/")]
async fn index(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }

        body.extend_from_slice(&chunk);
    }
    let obj = serde_json::from_slice::<Person>(&body)?;
    Ok(HttpResponse::Ok().json(obj))

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{HttpServer, App};

    HttpServer::new(|| {
        App::new()
        .service(index)
    })
    .bind(("::", 80))?
    .run()
    .await
}