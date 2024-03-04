use bookstore_api::run;

async fn spawn_app() {
    let server = run().expect("Failed to bind address");

    tokio::spawn(server);
}

#[tokio::test]
async fn test_health_check() {
    spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
