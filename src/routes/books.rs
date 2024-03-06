use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct BookData {
    title: String,
    author: String,
    genre: Option<String>,
}

pub async fn books() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn create_book(input: web::Json<BookData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let genre = match input.genre.clone() {
        Some(value) => value,
        None => "No genre".to_string(),
    };

    match sqlx::query!(
        r#"
        INSERT INTO books (id, title, author, genre, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        input.title,
        input.author,
        genre,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_book(input: web::Json<BookData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        DELETE FROM books
        WHERE title = $1 AND author = $2;
        "#,
        input.title,
        input.author,
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
