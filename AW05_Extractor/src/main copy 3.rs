/* JSON */
#![allow(non_snake_case)]
use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct InfoJson {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

async fn index(json: web::Json<InfoJson>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "username: {}, email: {}, sign_in_count: {}, is_active: {}",
        json.username, json.email, json.sign_in_count, json.active
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(1024)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                .into()
            });

        App::new().service(
            web::resource("/json")
            .app_data(json_config)
            .route(web::post().to(index))
        )
    })
    .bind(":::80")?
    .run()
    .await
}
