/* Url Dispatch (Path - HttpRequest) */
#![allow(non_snake_case)]
use actix_web::{get, App, HttpRequest, HttpServer, Responder};

#[get("/version/{v1}/{v2}")]
async fn index(req: HttpRequest) -> impl Responder {
    let v1 = req.match_info().get("v1").unwrap().parse::<u8>().unwrap();
    let v2 = req.match_info().query("v2").parse::<u8>().unwrap();

    let (v3, v4): (u8, u8) = req.match_info().load().unwrap();

    format!("v1: {} | v2: {} | v3: {} | v4: {}", v1, v2, v3, v4)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(":::80")?
        .run()
        .await
}
