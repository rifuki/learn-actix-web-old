/* STREAM DATA */
#![allow(non_snake_case)]
use actix_web::{HttpServer, App, get, web, HttpResponse, Error};
use futures::{future::ok, stream::once};

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(
        ok::<_, Error>(web::Bytes::from_static(b"RUST"))
    );

    HttpResponse::Ok().content_type("application/json").streaming(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(stream)
    })
    .bind(":::80")?
    .run()
    .await
}