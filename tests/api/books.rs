use crate::test_helpers::{drop_db, spawn_app};
use serde_json::Value;

#[tokio::test]
async fn books_index() {
    let app = spawn_app().await;
    app.create_author(r#"{"name":"JRR Tolkien", "nationality":"British"}"#.into())
        .await;
    app.create_book(
        r#"{"title":"Lord of the Rings", "author":"JRR Tolkien", "genre": "Fiction"}"#.into(),
    )
    .await;
    app.create_book(r#"{"title":"The Hobbit", "author":"JRR Tolkien", "genre": "Fiction"}"#.into())
        .await;

    let response = app.book_index().await;
    let parsed_response = response
        .json::<Value>()
        .await
        .expect("Failed to deserialize response body.");

    assert_eq!(parsed_response[0]["title"], "Lord of the Rings");
    assert_eq!(parsed_response[0]["author"], "JRR Tolkien");
    assert_eq!(parsed_response[0]["genre"], "Fiction");
    assert_eq!(parsed_response[1]["title"], "The Hobbit");
    assert_eq!(parsed_response[1]["author"], "JRR Tolkien");
    assert_eq!(parsed_response[1]["genre"], "Fiction");

    drop_db(app.db_name, app.db_url).await;
}

#[tokio::test]
async fn show_book() {
    let app = spawn_app().await;
    app.create_author(r#"{"name":"JRR Tolkien", "nationality":"British"}"#.into())
        .await;
    let create_response = app
        .create_book(
            r#"{"title":"Lord of the Rings", "author":"JRR Tolkien", "genre": "Fiction"}"#.into(),
        )
        .await;
    let response_body = create_response
        .json::<Value>()
        .await
        .expect("Failed to deserialize response body.");
    let book_id = response_body["book_id"]
        .as_str()
        .expect("Failed to extract author id from response.");

    let response = app.show_book(book_id.into()).await;
    let response_body2 = response
        .json::<Value>()
        .await
        .expect("Failed to deserialize response body.");

    assert_eq!(response_body2["id"], book_id);
    assert_eq!(response_body2["title"], "Lord of the Rings");
    assert_eq!(response_body2["author"], "JRR Tolkien");
    assert_eq!(response_body2["genre"], "Fiction");

    drop_db(app.db_name, app.db_url).await;
}

#[tokio::test]
async fn book_creation() {
    let app = spawn_app().await;
    app.create_author(r#"{"name":"JRR Tolkien", "nationality":"British"}"#.into())
        .await;

    let response = app
        .create_book(
            r#"{"title":"Lord of the Rings", "author":"JRR Tolkien", "genre": "Fiction"}"#.into(),
        )
        .await;
    let record = sqlx::query!(
        r#"SELECT  books.id,
            books.title,
            authors.name AS "authors_name",
            books.genre,
            books.created_at  FROM books JOIN authors ON books.author_id = authors.id"#
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch saved book.");

    assert!(response.status().is_success());
    assert_eq!(record.title, "Lord of the Rings");
    assert_eq!(record.authors_name, "JRR Tolkien");
    assert_eq!(record.genre, "Fiction");

    drop_db(app.db_name, app.db_url).await;
}

#[tokio::test]
async fn book_creation_with_incomplete_data() {
    let app = spawn_app().await;
    app.create_author(r#"{"name":"JRR Tolkien", "nationality":"British"}"#.into())
        .await;

    let response = app
        .create_book(r#"{"title":"Lord of the Rings", "author":"JRR Tolkien"}"#.into())
        .await;
    let record = sqlx::query!("SELECT * FROM books")
        .fetch_optional(&app.db_pool)
        .await
        .expect("Failed to fetch saved book.");

    assert!(response.status().is_client_error());
    assert!(record.is_none());

    drop_db(app.db_name, app.db_url).await;
}

#[tokio::test]
async fn book_deletion() {
    let app = spawn_app().await;
    app.create_author(r#"{"name":"JRR Tolkien", "nationality":"British"}"#.into())
        .await;
    let create_response = app
        .create_book(
            r#"{"title":"Lord of the Rings", "author":"JRR Tolkien", "genre": "Fiction"}"#.into(),
        )
        .await;
    let response_body = create_response
        .json::<Value>()
        .await
        .expect("Failed to deserialize response body.");
    let book_id = response_body["book_id"]
        .as_str()
        .expect("Failed to extract author id from response.");

    app.book_delete(format!(r#"{{"id": "{}"}}"#, book_id)).await;
    let record = sqlx::query!("SELECT * FROM books")
        .fetch_optional(&app.db_pool)
        .await
        .expect("Failed to fetch saved book.");

    assert!(record.is_none());

    drop_db(app.db_name, app.db_url).await;
}
