use crate::validations::author::NewAuthor;
use actix_web::{
    web::{Data, Json, Path},
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

pub async fn show_author(input: Path<String>, db_pool: Data<PgPool>) -> HttpResponse {
    let author_id = input.into_inner();

    match sqlx::query!(
        "SELECT id, name, nationality, created_at FROM authors WHERE id = $1",
        Uuid::parse_str(&author_id).unwrap_or_default()
    )
    .fetch_one(db_pool.get_ref())
    .await
    {
        Ok(author) => {
            let author_json = json!({
                "id": author.id,
                "name": author.name,
                "nationality": author.nationality,
                "created_at": author.created_at
            });

            HttpResponse::Ok().json(author_json)
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
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
        VALUES ($1, $2, $3)
        RETURNING id",
        new_author.name.as_ref(),
        new_author.nationality.as_ref(),
        Utc::now()
    )
    .fetch_one(db_pool.get_ref())
    .await
    {
        Ok(record) => HttpResponse::Ok().json(json!({
            "message": "Author created successfully!",
            "author_id": record.id
        })),
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
            true => HttpResponse::Ok().json(json!({"message": "Author deleted successfully!"})),
            false => {
                HttpResponse::NotFound().json(json!({"message": "Author to be deleted not found"}))
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
