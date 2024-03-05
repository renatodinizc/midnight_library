use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct BookData {
    title: String,
    author: String,
    genre: String,
}

pub async fn books() -> HttpResponse {
  HttpResponse::Ok().finish()
}

pub async fn create_book(input: web::Json<BookData>) -> HttpResponse {
  HttpResponse::Ok().finish()
}