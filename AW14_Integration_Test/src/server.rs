use std::net::TcpListener;
use actix_web::{dev::Server, HttpServer, App, middleware};

use crate::routes;

pub fn start(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
        .wrap(middleware::NormalizePath::default())
        .configure(routes::users)
    })
    .listen(listener)?
    .run();

    Ok(server)
}