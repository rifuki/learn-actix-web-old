/* Custom 404 Not Found*/
#![allow(non_snake_case)]
use actix_web::{Responder, HttpResponse, get, web, guard};

#[get("/", name="index")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("index page")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
        .service(index)
        .default_service(
            web::route()
            .guard(guard::Not(guard::Get()))
            .to(HttpResponse::NotAcceptable)
        )
    })
    .bind(("::", 80))?
    .run()
    .await
}
