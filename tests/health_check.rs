use bookstore_api::{configuration, run};
use sqlx::PgPool;
use std::net::TcpListener;

async fn spawn_app() -> String {
    let tcp_listener = TcpListener::bind("localhost:0").expect("Failed to bind random port");
    let address = tcp_listener
        .local_addr()
        .expect("Failed to get local address")
        .to_string();

    let config = configuration::get_configuration().expect("Failed to read configuration.");

    let db_connection =
        PgPool::connect(config.database.database_url().as_str())
            .await
            .expect("Failed to connect to Postgres.");

    let server = run(tcp_listener, db_connection).expect("Failed to bind address");
    tokio::spawn(server);

    address
}

#[tokio::test]
async fn health_check() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
