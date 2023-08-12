/* gk tau */
#![allow(non_snake_case)]
use actix_web::{guard, web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("index")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(
            web::resource("/").to(index)
        )
        .service(
            web::resource("/user/{name}")
                .name("user_detail")
                .guard(guard::Header("content-type", "application/json"))
                .route(web::get().to(|path: web::Path<String>| async move {
                    format!("name : {}", path.into_inner())
                }))
                .route(web::put().to(HttpResponse::NotModified))
        )
        .service(
            web::resource("/path")
            .route(
                web::route()
                .guard(guard::Get())
                .guard(guard::Header("content-type", "text/plain"))
                .to(HttpResponse::Ok)
            )
        )
    })
    .bind(":::80")?
    .run()
    .await
}
