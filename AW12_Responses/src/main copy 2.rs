/* Response Json */
#![allow(non_snake_case)]
use actix_web::{get, web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct Myobj {
    name: String
}

#[get("/{name}")]
async fn index(path: web::Path<String>) -> Result<impl Responder> {
    let obj = Myobj {name: path.into_inner().to_string() };

    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind(("::", 80))?
        .run()
        .await
}
