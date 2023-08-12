#![allow(non_snake_case)]
use actix_web::{HttpServer, App, web, Responder, HttpResponse};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>
}

async fn counter_route(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("Requested Number {}!\n\r", counter))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0)
    });

    HttpServer::new(move || {
        App::new()
        .app_data(counter.clone())
        .route("/", web::get().to(counter_route))
    })
    .bind(":::80")?
    .run()
    .await
}


