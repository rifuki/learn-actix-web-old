/* External Resources */
#![allow(non_snake_case)]

use actix_web::{get, Responder, HttpRequest};

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let url = req.url_for("pixiv", ["107636914"]).unwrap();

    url.to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{HttpServer, App};

    HttpServer::new(|| {
        App::new()
        // external resources
        .external_resource("pixiv", "https://pixiv.net/en/artworks/{pixiv_id}")
        .service(index)
    })
    .bind(("::", 80))?
    .run()
    .await
}