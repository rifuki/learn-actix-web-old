#![allow(non_snake_case)]
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login")
}

async fn logout() -> impl Responder {
    HttpResponse::Ok().body("logout")
}

#[get("/bitcoin")]
async fn bitcoin() -> impl Responder {
    HttpResponse::Ok().body("bitcoin")
}

async fn ethereum() -> impl Responder {
    HttpResponse::Ok().body("ethereum")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let scope_users = web::scope("/users")
            .service(login)
            .route("/logout", web::get().to(logout));
        let scope_crypto = web::scope("/crypto")
            .service(bitcoin)
            .route("/ethereum", web::get().to(ethereum));

        App::new().service(scope_users).service(scope_crypto)
    })
    .bind(":::80")?
    .run()
    .await
}
