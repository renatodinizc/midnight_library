use bookstore_api::{configuration, run};
use sqlx::{Executor, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

struct TestApp {
    address: String,
    db_pool: PgPool,
    db_name: Uuid,
}

async fn spawn_app() -> TestApp {
    let tcp_listener = TcpListener::bind("localhost:0").expect("Failed to bind random port");
    let address = tcp_listener
        .local_addr()
        .expect("Failed to get local address")
        .to_string();

    let (db_pool, db_name) = setup_db().await;

    let server = run(tcp_listener, db_pool.clone()).expect("Failed to bind address");
    tokio::spawn(server);

    TestApp {
        address,
        db_pool,
        db_name,
    }
}

async fn setup_db() -> (PgPool, Uuid) {
    let config = configuration::get_configuration().expect("Failed to read configuration.");

    let db_url = format!(
        "postgres://{}:{}@{}:{}",
        config.database.username,
        config.database.password,
        config.database.host,
        config.database.port,
    );

    let random_db_name = Uuid::new_v4();

    let db_pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to Postgres.");
    db_pool
        .execute(format!(r#"CREATE DATABASE "{}";"#, random_db_name).as_str())
        .await
        .expect("Failed to create database.");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate the database");

    (db_pool, random_db_name)
}

async fn drop_db(name: Uuid, db_pool: PgPool) {
    db_pool
        .execute(format!(r#"DROP DATABASE "{}";"#, name).as_str())
        .await
        .expect("Failed to create database.");
}

#[tokio::test]
async fn book_creation() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = r#"{"title":"Harry Potter and the philosopher's stone", "author":"JK Rowling", "genre": "Fiction"}"#;

    let response = client
        .post(format!("http://{}/books/create", app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    let record = sqlx::query!("SELECT * FROM books")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(record.title, "Harry Potter and the philosopher's stone");
    assert_eq!(record.author, "JK Rowling");
    assert_eq!(record.genre, "Fiction");

    drop_db(app.db_name, app.db_pool).await;
}

#[tokio::test]
async fn book_creation_with_incomplete_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = r#"{"title":"Harry Potter and the philosopher's stone"}"#;

    let response = client
        .post(format!("http://{}/books/create", app.address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_client_error());

    drop_db(app.db_name, app.db_pool).await;
}

#[tokio::test]
async fn books_index() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/books", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    drop_db(app.db_name, app.db_pool).await;
}
