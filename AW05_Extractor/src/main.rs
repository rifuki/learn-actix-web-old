/* FORM */
#![allow(non_snake_case)]
use actix_web::{HttpServer, App, post, web, Responder, HttpResponse, http::StatusCode};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct InfoForm {
    song_title: String,
    views: u64,
    producer: String,
    singer: String
}


#[post("/form")]
async fn index(info: web::Form<InfoForm>) -> impl Responder {
    HttpResponse::build(StatusCode::OK).body(format!("{} | {} | {} | {}", info.song_title, info.producer, info.singer, info.views))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(index)
    })
    .bind(":::80")?
    .run()
    .await
}