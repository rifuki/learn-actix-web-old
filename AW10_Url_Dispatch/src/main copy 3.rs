/* Macro Route Identifier */
#![allow(non_snake_case)]

use actix_web::{web, get, Responder, HttpRequest, HttpResponse};

// proc-macro identifier
#[get("/show", name = "show_users") ]
async fn show_users() -> impl Responder {
    HttpResponse::Ok().body("show users")
}

async fn generate_url(req: HttpRequest) -> impl Responder {
    let url = req.url_for("show_users", [""]);

    match url {
        Ok(url) => HttpResponse::Ok().body(format!("generate_url: {}", url)),
        Err(_) => HttpResponse::InternalServerError().body("failed to generate url")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{HttpServer, App};

    HttpServer::new(|| {
        App::new()
        .service(
            web::scope("/users")
            .service(show_users)
        )
        .route("/generate", web::get().to(generate_url))
    })
    .bind(("::", 80))?
    .run()
    .await
}