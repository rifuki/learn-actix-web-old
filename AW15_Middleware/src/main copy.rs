/* Middleware */
#![allow(non_snake_case)]
use actix_web::{get, HttpResponse};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error>{
    use actix_web::{HttpServer, App, middleware};

    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .wrap(middleware::Compress::default())
        .wrap(middleware::NormalizePath::trim())
        .service(index)
    })
    .bind(":::80")?
    .run()
    .await
}
