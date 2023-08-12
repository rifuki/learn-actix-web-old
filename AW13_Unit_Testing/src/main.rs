/* Unit Testing */
#![allow(non_snake_case)]
fn main() {}

use actix_web::{http::header, HttpRequest, HttpResponse};

async fn index(req: HttpRequest) -> HttpResponse {
    if let Some(_) = req.headers().get(header::CONTENT_TYPE) {
        HttpResponse::Ok().into()
    } else {
        HttpResponse::BadRequest().into()
    }
}

async fn echo(req_body: String) -> HttpResponse {
    HttpResponse::Ok().body(req_body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{header, StatusCode},
        test, web::{self}, App,
    };

    #[actix_web::test]
    async fn test_index_ok_v1() {
        let req = test::TestRequest::default()
            .insert_header((header::CONTENT_TYPE, header::ContentType::plaintext()))
            .to_http_request();

        let resp = index(req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_index_ok_v2() {
        let app = App::new().route("/", web::patch().to(index));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::patch()
            .uri("/")
            .insert_header((
                header::CONTENT_TYPE,
                header::ContentType::form_url_encoded(),
            ))
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_index_fails_v1() {
        let req = test::TestRequest::default().to_http_request();

        let resp = index(req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_index_fails_v2() {
        let app = App::new().route("/", web::delete().to(index));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::delete().to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_echo() {
        let app = App::new().route("/echo", web::patch().to(echo));
        let mut app = test::init_service(app).await;

        let req_body = "name=\"Hatsune Miku\"";
        let req = test::TestRequest::patch()
        .uri("/echo")
        .insert_header((header::CONTENT_TYPE, header::ContentType::form_url_encoded()))
        .set_payload(req_body)
        .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let resp_body = test::read_body(resp).await;
        assert_eq!(resp_body, req_body);
    }

}
