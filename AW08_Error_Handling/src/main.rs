/* Logger */
#![allow(non_snake_case)]
use actix_web::{HttpServer, App, Responder, web};
use log::error;

async fn register() -> impl Responder {
    error!("{}", 3+3);
    "hello"
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
        .service(
            web::resource("/")
            .to(register)
        )
    })
    .bind(":::80")?
    .run()
    .await
}
