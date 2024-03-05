use bookstore_api::run;
use std::net::TcpListener;

async fn spawn_app() -> String {
    let tcp_listener = TcpListener::bind("localhost:0").expect("Failed to bind random port");
    let address = tcp_listener
        .local_addr()
        .expect("Failed to get local address")
        .to_string();

    let server = run(tcp_listener).expect("Failed to bind address");
    tokio::spawn(server);

    address
}

#[tokio::test]
async fn test_book_creation() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let body = r#"{"title":"Harry Potter and the philosopher's stone", "author":"JK Rowling", "genre": "Fiction"}"#;

    let response = client
        .post(format!("http://{}/books/create", address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn test_books_index() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/books", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
