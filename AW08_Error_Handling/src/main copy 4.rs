/* Generics Error */
#![allow(non_snake_case)]
use actix_web::{
    body::BoxBody,
    error,
    http::{header::ContentType, StatusCode},
    post, web, App, HttpResponse, HttpServer,
};
use derive_more::Display;
use serde::Deserialize;

#[derive(Display, Debug)]
enum UserError {
    #[display(fmt = "an internal error occurs. please try again later.")]
    InternalError,
    #[display(fmt = "invalid input provided.")]
    InvalidInput,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status_code = self.status_code();
        let message = self.to_string();
        HttpResponse::build(status_code)
            .insert_header(ContentType::html())
            .body(message)
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::InvalidInput => StatusCode::BAD_REQUEST,
        }
    }
}

fn do_thing_that_fails() -> Result<&'static str, UserError> {
    Err(UserError::InternalError)
}

#[derive(Deserialize)]
struct MyData {
    value: String,
}

#[post("/process")]
async fn process(data: web::Json<MyData>) -> Result<&'static str, UserError> {
    if data.value.is_empty() {
        return Err(UserError::InvalidInput)?;
    }

    do_thing_that_fails().map_err(|e| e)?;

    Ok("asiap")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(process))
        .bind(":::80")?
        .run()
        .await
}
