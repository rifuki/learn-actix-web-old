#![allow(non_snake_case)]
use actix_web::{main, HttpServer, App, Responder, HttpResponse, web, get};

struct AppState {
    app_name: String
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

async fn manual_hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("POST! {}!", app_name)
}

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .app_data(web::Data::new(AppState {
            app_name: String::from("Rust Lang")
        }))
        .service(
            web::scope("/app")
            .service(index)
        )
        .route("/post", web::post().to(manual_hello))
    })
    .bind(("::", 80))?
    .run()
    .await
}
