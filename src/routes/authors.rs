use crate::validations::author::NewAuthor;
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

pub async fn authors_index(db_pool: Data<PgPool>) -> HttpResponse {
    let rows = sqlx::query!("SELECT id, name, nationality, created_at FROM authors")
        .fetch_all(db_pool.get_ref())
        .await
        .expect("Failed to fetch saved authors.");

    let authors: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.id,
                "name": row.name,
                "nationality": row.nationality,
                "created_at": row.created_at
            })
        })
        .collect();

    HttpResponse::Ok().json(authors)
}

#[derive(Serialize, Deserialize)]
pub struct NewAuthorData {
    pub name: String,
    pub nationality: String,
}

pub async fn create_author(input: Json<NewAuthorData>, db_pool: Data<PgPool>) -> HttpResponse {
    let new_author: NewAuthor = match input.0.try_into() {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error),
    };

    match sqlx::query!(
        "INSERT INTO authors (name, nationality, created_at)
        VALUES ($1, $2, $3)",
        new_author.name.as_ref(),
        new_author.nationality.as_ref(),
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body("Author created successfully!\n"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
pub struct AuthorId {
    id: String,
}

pub async fn delete_author(input: Json<AuthorId>, db_pool: Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        "DELETE FROM authors WHERE id = $1",
        Uuid::parse_str(&input.id).unwrap_or_default(),
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(result) => match result.rows_affected() == 1 {
            true => HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("Author deleted successfully!\n"),
            false => HttpResponse::NotFound()
                .content_type(ContentType::plaintext())
                .body("Author to be deleted not found!\n"),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
