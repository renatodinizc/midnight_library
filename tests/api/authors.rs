use crate::test_helpers::{drop_db, spawn_app};
use serde_json::Value;

#[tokio::test]
async fn authors_index() {
    let app = spawn_app().await;
    app.create_author(r#"{"name":"JRR Tolkien", "nationality":"British"}"#.into())
        .await;
    app.create_author(r#"{"name":"Herman Melville", "nationality":"American"}"#.into())
        .await;

    let response = app.author_index().await;
    let parsed_response = response
        .json::<Value>()
        .await
        .expect("Failed to deserialize response body.");

    assert_eq!(parsed_response[0]["name"], "JRR Tolkien");
    assert_eq!(parsed_response[0]["nationality"], "British");
    assert_eq!(parsed_response[1]["name"], "Herman Melville");
    assert_eq!(parsed_response[1]["nationality"], "American");

    drop_db(app.db_name, app.db_url).await;
}

#[tokio::test]
async fn show_author() {
    let app = spawn_app().await;
    let create_response = app
        .create_author(r#"{"name":"JRR Tolkien", "nationality":"British"}"#.into())
        .await;
    let response_body = create_response
        .json::<Value>()
        .await
        .expect("Failed to deserialize response body.");
    let author_id = response_body["author_id"]
        .as_str()
        .expect("Failed to extract author id from response.");

    let response = app.show_author(author_id.into()).await;
    let response_body2 = response
        .json::<Value>()
        .await
        .expect("Failed to deserialize response body.");

    assert_eq!(response_body2["name"], "JRR Tolkien");
    assert_eq!(response_body2["nationality"], "British");
    assert_eq!(response_body2["id"], author_id);

    drop_db(app.db_name, app.db_url).await;
}

#[tokio::test]
async fn author_creation() {
    let app = spawn_app().await;
    let body = r#"{"name":"JRR Tolkien", "nationality":"British"}"#;

    let response = app.create_author(body.into()).await;
    let record = sqlx::query!("SELECT * FROM authors")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved author.");

    assert!(response.status().is_success());
    assert_eq!(record.name, "JRR Tolkien");
    assert_eq!(record.nationality, "British");

    drop_db(app.db_name, app.db_url).await;
}

#[tokio::test]
async fn author_creation_with_incomplete_data() {
    let app = spawn_app().await;
    let body = r#"{"name":"JRR Tolkien"}"#;

    let response = app.create_author(body.into()).await;
    let record = sqlx::query!("SELECT * FROM authors")
        .fetch_optional(&app.db_pool)
        .await
        .expect("Failed to fetch saved author.");

    assert!(response.status().is_client_error());
    assert!(record.is_none());

    drop_db(app.db_name, app.db_url).await;
}

#[tokio::test]
async fn author_deletion() {
    let app = spawn_app().await;
    let response = app
        .create_author(r#"{"name":"JRR Tolkien", "nationality":"British"}"#.into())
        .await;
    let response_body = response
        .json::<Value>()
        .await
        .expect("Failed to deserialize response body.");
    let author_id = response_body["author_id"]
        .as_str()
        .expect("Failed to extract author id from response.");

    app.delete_author(format!(r#"{{"id": "{}"}}"#, author_id))
        .await;
    let record = sqlx::query!("SELECT * FROM authors")
        .fetch_optional(&app.db_pool)
        .await
        .expect("Failed to fetch saved author.");

    assert!(record.is_none(), "Record was not deleted successfully.");

    drop_db(app.db_name, app.db_url).await;
}
