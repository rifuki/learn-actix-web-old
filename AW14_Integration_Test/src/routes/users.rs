use crate::controllers;
use actix_web::{middleware, web};

pub fn users(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap(middleware::NormalizePath::trim())
            .route("", web::get().to(controllers::get_users))
            .route("/{user}", web::post().to(controllers::create_user))
            .route("", web::put().to(controllers::edit_user)),
    );
}
