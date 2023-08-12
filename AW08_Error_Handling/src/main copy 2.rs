/* Override ResponseError Methods  */
#![allow(non_snake_case)]
use actix_web::{HttpServer, App, error, get, body::BoxBody, HttpResponse, http::{StatusCode, header::ContentType} };
use derive_more::Display;

#[derive(Debug, Display)]
enum MyCustomError {
    #[display(fmt = "{}", StatusCode::INTERNAL_SERVER_ERROR)]
    InternalError,
    #[display(fmt = "{}", StatusCode::BAD_REQUEST)]
    BadDataClient,
    #[display(fmt = "{}", StatusCode::GATEWAY_TIMEOUT)]
    Timeout
}

impl error::ResponseError for MyCustomError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).insert_header(ContentType::json()).body(format!(r#"{{"message": {}}}"#, self.to_string()))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyCustomError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyCustomError::BadDataClient => StatusCode::BAD_REQUEST,
            _ => StatusCode::GATEWAY_TIMEOUT
        }
    }
}

#[get("/internal")]
async fn internal_error() -> Result<&'static str, impl error::ResponseError> {
    Err(MyCustomError::InternalError)
}

#[get("/bad")]
async fn bad_request() -> Result<&'static str, MyCustomError> {
    Err(MyCustomError::BadDataClient)
}

#[get("/timeout")]
async fn timeout() -> Result<&'static str, impl error::ResponseError> {
    Err(MyCustomError::Timeout)
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
