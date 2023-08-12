use std::net::TcpListener;
use AW14_Integration_Test::server;
use reqwest::StatusCode;

struct MyServer {
    address: String
}

async fn start_server(listener: TcpListener) -> MyServer {
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let server = server::start(listener).unwrap();

    actix_web::rt::spawn(server);

    MyServer { address }
}

#[actix_web::test]
async fn test_server_get_users_ok() {
    let listener = TcpListener::bind(("::", 0)).unwrap();
    let server = start_server(listener).await;

    let client = reqwest::Client::new();
    let resp = client.get(format!("{}/users", server.address)).header("content-type", "application/json").send().await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_server_create_user_ok() {
    let listener = TcpListener::bind(("::", 0)).unwrap();
    let server = start_server(listener).await;

    let client = reqwest::Client::new();
    let resp = client.post(format!("{}/users/rifuki", server.address)).header("content-type", "application/json").send().await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
}