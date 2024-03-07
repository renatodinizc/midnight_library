use actix_web::{http::header::ContentType, web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use super::super::models::{Author, AuthorId};

pub async fn authors_index(db_pool: web::Data<PgPool>) -> HttpResponse {
    let authors = sqlx::query_as!(
        Author,
        r#"SELECT id, name, nationality, created_at FROM authors"#
    )
    .fetch_all(db_pool.get_ref())
    .await
    .expect("Failed to fetch saved authors.");

    HttpResponse::Ok().json(authors)
}

pub async fn create_author(input: web::Json<Author>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO authors (name, nationality, created_at)
        VALUES ($1, $2, $3)
        "#,
        input.name,
        input.nationality,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body("Author created successfully!\n"),
        Err(_e) => HttpResponse::InternalServerError().body("Couldn't create specified author.\n"),
    }
}

pub async fn delete_author(input: web::Json<AuthorId>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        DELETE FROM authors
        WHERE id = $1;
        "#,
        Uuid::parse_str(&input.id).unwrap_or_default(),
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body("Author created successfully!\n"),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}
