use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct BookData {
    title: String,
    author: String,
    genre: String,
}

pub async fn books() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn create_book(input: web::Json<BookData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let uuid = Uuid::new_v4();
    match sqlx::query!(
        r#"
        INSERT INTO books (id, title, author, genre, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        uuid,
        input.title,
        input.author,
        input.genre,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => {
            println!(
                "Book created! Details: title: {}, author: {}, genre: {}, book Uuid: {}",
                input.title, input.author, input.genre, uuid
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(serde::Deserialize)]
pub struct BookDeletionData {
    id: String,
}

pub async fn delete_book(
    input: web::Json<BookDeletionData>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
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
