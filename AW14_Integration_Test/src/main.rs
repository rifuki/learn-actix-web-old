#![allow(non_snake_case)]

use std::net::TcpListener;
use AW14_Integration_Test::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 80)).unwrap();

    let server = server::start(listener)?.await;

    server
}