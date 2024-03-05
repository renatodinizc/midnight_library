use bookstore_api::{configuration, run};
use chrono::Utc;
use sqlx::{Executor, PgPool};
use std::net::TcpListener;

struct TestApp {
    address: String,
}

async fn spawn_app() -> TestApp {
    let tcp_listener = TcpListener::bind("localhost:0").expect("Failed to bind random port");
    let address = tcp_listener
        .local_addr()
        .expect("Failed to get local address")
        .to_string();

    let db_pool = setup_db().await;

    let server = run(tcp_listener, db_pool.clone()).expect("Failed to bind address");
    tokio::spawn(server);

    TestApp { address }
}

async fn setup_db() -> PgPool {
    let config = configuration::get_configuration().expect("Failed to read configuration.");

    let db_url = format!(
        "postgres://{}:{}@{}:{}",
        config.database.username,
        config.database.password,
        config.database.host,
        config.database.port,
    );

    let db_pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to Postgres.");
    db_pool
        .execute(format!(r#"CREATE DATABASE "{}";"#, Utc::now()).as_str())
        .await
        .expect("Failed to create database.");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate the database");

    db_pool
}

#[tokio::test]
async fn health_check() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
