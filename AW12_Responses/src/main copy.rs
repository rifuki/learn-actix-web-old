/* Response Body */
#![allow(non_snake_case)]

use actix_web::{get, http::header::ContentType, HttpResponse};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-HDR", "sample"))
        .body("data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind(("::", 80))?
        .run()
        .await
}
