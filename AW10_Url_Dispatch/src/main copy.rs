/* Resource Identifier */
#![allow(non_snake_case)]
use actix_web::{HttpServer, App, get, Result, web, guard, HttpResponse, HttpRequest, http::header};

#[get("/test")]
async fn test(req: HttpRequest) -> Result<HttpResponse> {
    let url = req.url_for("foo", ["1", "2", "3"])?;

    Ok(HttpResponse::Found().insert_header((header::LOCATION, url.as_str())).finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(
            web::resource("/test/{a}/{b}/{c}")
            // identifier
            .name("foo")
            .guard(guard::Get())
            .to(HttpResponse::Ok)
        )
        .service(test)
    })
    .bind(("::", 80))?
    .run()
    .await
}