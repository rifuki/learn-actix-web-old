/* Custom Error Response */
#![allow(non_snake_case)]
use actix_web::{HttpServer, App, error, get};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "Invalid Name: {}", name)]
struct MyError {
    name: &'static str,
}

impl error::ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<&'static str, MyError> {
    Err( MyError { name: "rifuki"} )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(index)
    })
    .bind(":::80")?
    .run()
    .await
}