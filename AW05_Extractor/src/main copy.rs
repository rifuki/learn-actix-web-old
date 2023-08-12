/* PATH */
#![allow(non_snake_case)]
use actix_web::{HttpServer, App, web, Result, get, HttpRequest};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    username: String
}

#[get("/v1/{user_id}/{username}")]
async fn index1(info: web::Path<Info>) -> Result<String> {
    Ok(format!("koncichiwa {}, your id: {} ", info.username, info.user_id))
}

#[get("/v2/{user_id}/{username}")]
async fn index2(info: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, username) = info.into_inner();
    Ok(format!("konbanwa {}, your id: {}", user_id, username))
}

async fn index3(req: HttpRequest) -> Result<String> {
    let user_id = req.match_info().query("user_id").parse::<u32>().unwrap();
    // let user_id: u32 = req.match_info().get("user_id").unwrap().parse().unwrap();
    let username = req.match_info().get("username").unwrap().parse::<String>().unwrap();

    Ok(format!("oyasumi {}, your id: {}", username, user_id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(
            web::scope("/users")
            .service(index1)
            .service(index2)
            .route("/v3/{user_id}/{username}", web::get().to(index3))
        )
    })
    .bind(":::80")?
    .run()
    .await
}