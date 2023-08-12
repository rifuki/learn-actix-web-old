#![allow(non_snake_case)]
use actix_web::{guard, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::get("/hey")]
async fn hey() -> impl Responder {
    HttpResponse::Ok().body("hey")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/app")
                .guard(guard::Get())
                .service(hey)
        )
        .route("/", web::get().to(|| async { "root"}))
    })
    .bind(":::80")?
    .run()
    .await
}
