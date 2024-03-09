use actix_web::{
    http::header::ContentType,
    web::{Data, Json},
    HttpResponse,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::validations::book::NewBook;

pub async fn books_index(db_pool: Data<PgPool>) -> HttpResponse {
    let rows = sqlx::query!(
        r#"
        SELECT
            books.id,
            books.title,
            authors.name AS "authors_name",
            books.genre,
            books.created_at
        FROM books
        JOIN authors ON books.author_id = authors.id
        "#
    )
    .fetch_all(db_pool.get_ref())
    .await
    .expect("Failed to fetch saved books.");

    let books: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.id,
                "title": row.title,
                "author": row.authors_name,
                "genre": row.genre,
                "created_at": row.created_at
            })
        })
        .collect();

    HttpResponse::Ok().json(books)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewBookData {
    pub title: String,
    pub author: String,
    pub genre: String,
}

pub async fn create_book(input: Json<NewBookData>, db_pool: Data<PgPool>) -> HttpResponse {
    let new_book: NewBook = match input.0.try_into() {
        Ok(value) => value,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };

    let author = match sqlx::query!(
        r#"SELECT id, name, nationality, created_at FROM authors WHERE name = $1"#,
        new_book.author.as_ref()
    )
    .fetch_one(db_pool.get_ref())
    .await
    {
        Ok(author) => author,
        Err(_e) => {
            return HttpResponse::BadRequest().body("Couldn't find specified author of the book.\n")
        }
    };

    match sqlx::query!(
        r#"
        INSERT INTO books (title, genre, author_id, created_at)
        VALUES ($1, $2, $3, $4)
        "#,
        new_book.title.as_ref(),
        new_book.genre.as_ref(),
        author.id,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body("Book created successfully!\n"),
        Err(_e) => HttpResponse::InternalServerError().body("Couldn't create specified book.\n"),
    }
}

#[derive(Deserialize)]
pub struct BookId {
    id: String,
}

pub async fn delete_book(input: Json<BookId>, db_pool: Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        DELETE FROM books
        WHERE id = $1;
        "#,
        Uuid::parse_str(&input.id).unwrap_or_default(),
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body("Book deleted successfully!\n"),
        Err(_e) => {
            HttpResponse::InternalServerError().body("Couldn't find specified book to delete.\n")
        }
    }
}
