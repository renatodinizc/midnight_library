use crate::test_helpers::{drop_db, spawn_app};

#[tokio::test]
async fn user_creation() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(format!("http://{}/users/create", app.address))
        .header("Content-Type", "application/json")
        .body(r#"{"name":"renato", "email":"example@email.com"}"#)
        .send()
        .await
        .expect("Failed to execute request.");

    let record = sqlx::query!("SELECT * FROM users")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert!(response.status().is_success());
    assert_eq!(record.name, "renato");
    assert_eq!(record.email, "example@email.com");

    drop_db(app.db_name, app.db_url).await;
}

#[tokio::test]
async fn user_creation_with_invalid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(format!("http://{}/users/create", app.address))
        .header("Content-Type", "application/json")
        .body(r#"{"name":"renato", "email":"example.com}"#)
        .send()
        .await
        .expect("Failed to execute request.");
    let record = sqlx::query!("SELECT * FROM users")
        .fetch_optional(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert!(response.status().is_client_error());
    assert!(record.is_none());

    drop_db(app.db_name, app.db_url).await;
}
