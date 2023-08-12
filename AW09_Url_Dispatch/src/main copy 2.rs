/* Url Dispatch (Path - Tuple) */
#![allow(non_snake_case)]
use actix_web::{HttpServer, App, Responder, HttpResponse, web, guard};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("index")
}

async fn show_users() -> impl Responder {
    HttpResponse::Ok().body("show users")
}

async fn show_user(path: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("user id: {}", path.into_inner().0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(
            web::scope("/api")
            .service(
                web::resource("/hello")
                .route(
                    web::route()
                    .guard(guard::Get())
                    .to(index)
                )
            )
            .service(web::resource("/users").to(show_users))
            .service(web::resource("/users/{name}").to(show_user))
        )
    })
    .bind(":::80")?
    .run()
    .await
}