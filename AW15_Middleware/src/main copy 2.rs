/* Simple Middleware */
#![allow(non_snake_case)]
use actix_web::{get, HttpResponse, dev::Service};
// needs futures_util
use futures_util::future::FutureExt;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error>{
    use actix_web::{HttpServer, App, middleware};

    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .wrap_fn(| req, srv | {
            println!("Hi from start. you request {}", req.path());

            srv.call(req).map(| res | {
                println!("Hi from response.");
                res
            })
        } )
        .service(index)
    })
    .bind(":::80")?
    .run()
    .await
}
