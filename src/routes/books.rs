use actix_web::{http::header::ContentType, web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Book {
    title: String,
    author: String,
    genre: String,
}

pub async fn books(db_pool: web::Data<PgPool>) -> HttpResponse {
    let books = sqlx::query_as!(Book, r#"SELECT title, author, genre FROM books"#)
        .fetch_all(db_pool.get_ref())
        .await
        .expect("Failed to fetch saved books.");

    HttpResponse::Ok().json(books)
}

pub async fn create_book(input: web::Json<Book>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO books (title, author, genre, created_at)
        VALUES ($1, $2, $3, $4)
        "#,
        input.title,
        input.author,
        input.genre,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body("Book created successfully!\n"),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
pub struct BookId {
    id: String,
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
        Ok(_) => {
            println!("Book deleted!book Uuid: {}", input.id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
