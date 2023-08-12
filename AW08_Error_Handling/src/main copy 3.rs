/* Error Helpers Function */
#![allow(non_snake_case)]
use actix_web::{error, get, App, HttpResponse, HttpServer, Result};

#[derive(Debug)]
struct MyError {
    name: &'static str,
}

#[get("/internal")]
async fn internal_error() -> Result<&'static str> {
    let result: Result<&'static str, MyError> = Err(MyError { name: "Aozora" });

    Ok(result.map_err(|e| error::ErrorInternalServerError(e.name))?)
}

#[get("/bad")]
async fn bad_request() -> Result<String> {
    let result: Result<String, MyError> = Err(MyError { name: "Iuchi" });

    Ok(result.map_err(|e| error::ErrorBadRequest(e.name))?)
}

#[get("/timeout")]
async fn timeout() -> Result<HttpResponse> {
    let result: Result<HttpResponse, MyError> = Err(MyError { name: "Setsuna" });

    Ok(result.map_err(|e| error::ErrorGatewayTimeout(e.name))?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(internal_error)
            .service(bad_request)
            .service(timeout)
    })
    .bind(":::80")?
    .run()
    .await
}
