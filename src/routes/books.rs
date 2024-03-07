use actix_web::{http::header::ContentType, web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use super::super::models::Author;
use super::super::models::{Book, BookId, CreateBookData};

pub async fn books_index(db_pool: web::Data<PgPool>) -> HttpResponse {
    let books = sqlx::query_as!(
        Book,
        r#"SELECT id, title, author_id, genre, created_at FROM books"#
    )
    .fetch_all(db_pool.get_ref())
    .await
    .expect("Failed to fetch saved books.");

    HttpResponse::Ok().json(books)
}

pub async fn create_book(
    input: web::Json<CreateBookData>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    let author = match sqlx::query_as!(
        Author,
        r#"SELECT id, name, nationality, created_at FROM authors WHERE name = $1"#,
        input.author
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
        input.title,
        input.genre,
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

pub async fn delete_book(input: web::Json<BookId>, db_pool: web::Data<PgPool>) -> HttpResponse {
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
