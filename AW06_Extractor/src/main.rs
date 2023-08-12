#![allow(non_snake_case)]
use actix_web::{HttpServer, App, web, Responder, HttpResponse};
use serde_json::json;
use std::sync::{Arc, Mutex};
use actix_cors::Cors;


#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<i32>>
}

async fn show_counter(data: web::Data<AppState>) -> impl Responder {
    let counter = *data.counter.lock().unwrap();
    let res = json!({
        "counter": counter
    });
    HttpResponse::Ok().content_type("application/json").json(res)
}

async fn increment_counter(data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    HttpResponse::Found().insert_header(("Location", "/counter/")).finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState {
        counter: Arc::new(Mutex::new(Default::default()))
    };
    println!("server is beating up at http://{}", local_ip_address::local_ip().unwrap());

    HttpServer::new(move || {
        App::new()
        .wrap(
            Cors::default()
            .allowed_origin("http://0.0.0.0:5500") 
            .allowed_origin("http://192.168.233.151:5500")
        )
        .service(
            web::scope("/counter")
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(show_counter))
            .route("/", web::post().to(increment_counter))
        )
    })
    .bind(":::80")?
    .run()
    .await
}