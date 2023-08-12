use actix_web::{HttpResponse, web, HttpRequest};

pub async fn get_users(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("GET USERS")
}

pub async fn create_user(path: web::Path<(String,)>) -> HttpResponse {
    HttpResponse::Created().body(format!("user {} created", path.into_inner().0))
}

pub async fn edit_user() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, http::StatusCode, App};

    #[actix_web::test]
    async fn test_get_users() {
        let req = test::TestRequest::get()
        .to_http_request();
        let resp = get_users(req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_create_user() {
        let app = App::new().route("/users/{id}", web::post().to(create_user));
        let mut app = test::init_service(app).await;

        let request = test::TestRequest::post()
        .uri("/users/rifuki")
        .insert_header(("content-type", "text/plain"))
        .to_request();

        let resp = test::call_service(&mut app, request).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

}
