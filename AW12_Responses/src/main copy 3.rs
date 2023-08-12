/* Middleware Compresion */
#![allow(non_snake_case)]
use actix_web::{get, middleware,HttpResponse, http::header::ContentEncoding};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
    // disable compression
        .insert_header(ContentEncoding::Identity)
        .body("data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index)
    })
    .bind(("::", 80))?
    .run()
    .await
}
