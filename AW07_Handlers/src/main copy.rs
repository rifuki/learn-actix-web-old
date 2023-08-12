/* CUSTOM RESPONDER */
#![allow(non_snake_case)]
use actix_web::{HttpServer, App, web, Responder, HttpResponse, HttpRequest, body::BoxBody, http::header::ContentType};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct MyCustomReturn<T: Serialize> {
    item: T
}

impl<T> Responder for MyCustomReturn<T> where T: Serialize{
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string_pretty(&self).unwrap();

        HttpResponse::Ok().insert_header(("content-type", "application/json")).body(body)
    }
}

async fn index() -> impl Responder { 
    MyCustomReturn { item: "Hatsune Miku" }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(index))
    })
    .bind(":::80")?
    .run()
    .await
}
