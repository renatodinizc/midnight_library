use crate::test_helpers::{drop_db, spawn_app};

#[tokio::test]
async fn user_creation() {
    let app = spawn_app().await;

    let response = app
        .create_user(r#"{"name":"Richard", "email":"example@email.com"}"#.into())
        .await;
    let record = sqlx::query!("SELECT * FROM users")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert!(response.status().is_success());
    assert_eq!(record.name, "Richard");
    assert_eq!(record.email, "example@email.com");

    drop_db(app.db_name, app.db_url).await;
}

#[tokio::test]
async fn user_creation_with_invalid_data() {
    let app = spawn_app().await;

    let response = app
        .create_user(r#"{"name":"Richard", "email":"example.com"}"#.into())
        .await;
    let record = sqlx::query!("SELECT * FROM users")
        .fetch_optional(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert!(response.status().is_client_error());
    assert!(record.is_none());

    drop_db(app.db_name, app.db_url).await;
}
