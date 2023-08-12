/* QUERY */
#![allow(non_snake_case)]
use actix_web::{HttpServer, App, web, get};
use serde::Deserialize;

#[derive(Deserialize)]
struct InfoQuery {
    username: String,
    age: u32,
    fav: String
}

#[get("/query")]
async fn index(query: web::Query<InfoQuery>) -> String {
    format!("Welcome {}, age: {}, waifu: {}!", query.username, query.age, query.fav)
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