#![allow(non_snake_case)]
use actix_web::{HttpServer, App, web};
use A3_Application_Builder::{auth, security};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .configure(security::scope_security)
        .service(web::scope("/api").configure(auth::scope_auth))
        .route("/", web::get().to(|| async { actix_web::HttpResponse::Ok().body("root\n")}))
    })
    .bind(":::80")?
    .run()
    .await
}