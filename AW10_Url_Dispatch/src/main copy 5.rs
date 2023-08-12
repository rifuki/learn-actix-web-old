/* Custom Guard */
#![allow(non_snake_case)]

use actix_web::{
    guard::{self, Guard, GuardContext},
    http::header,
    web, HttpResponse,
};

struct ContentTypeHeader;
impl Guard for ContentTypeHeader {
    fn check(&self, ctx: &GuardContext) -> bool {
        ctx.head().headers().contains_key(header::CONTENT_TYPE)
    }
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/", web::route().guard(ContentTypeHeader).to(index))
            .route(
                "/notallowed",
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(HttpResponse::MethodNotAllowed),
            )
    })
    .bind(("::", 80))?
    .run()
    .await
}
