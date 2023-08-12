/* HANDLE 2 TYPE FOR RESPONSE */
#![allow(non_snake_case)]
use actix_web::{HttpServer, App, get, web, HttpResponse, Either, Error};

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

fn is_valid() -> bool {
    // do something here
    false
}

async fn index() -> RegisterResult {
    if is_valid() {
        Either::Left(HttpResponse::BadRequest().body("bad data"))
    } else {
        Either::Right(Ok("success"))
    }
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