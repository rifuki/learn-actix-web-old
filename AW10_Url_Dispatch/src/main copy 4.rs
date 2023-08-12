/* Path Normalize */
#![allow(non_snake_case)]

use actix_web::{middleware, web, HttpResponse, Responder};

async fn show_users() -> impl Responder {
    HttpResponse::Ok().body("show users")
}
async fn show_user() -> impl Responder {
    HttpResponse::Ok().body("user detail")
}
async fn add_user() -> impl Responder {
    HttpResponse::Created().body("user created")
}
async fn edit_user() -> impl Responder {
    HttpResponse::Ok().body("user updateded")
}
async fn delete_user() -> impl Responder {
    HttpResponse::Found().body("user delete")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .service(
                web::scope("/users")
                    .route("", web::get().to(show_users))
                    .route("", web::post().to(add_user))
                    .route("/{id}", web::get().to(show_user))
                    .route("/{id}", web::put().to(edit_user))
                    .route("/{id}", web::delete().to(delete_user)),
            )
    })
    .bind(("::", 80))?
    .run()
    .await
}
