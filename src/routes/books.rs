use actix_web::web;

#[derive(serde::Deserialize)]
pub struct BookData {
    title: String,
    author: String,
    genre: String,
}

pub async fn books() -> String {
  "books endpoint".to_string()
}

pub async fn create_book(input: web::Json<BookData>) -> String {
  format!("title: {}, author: {}, genre: {}", input.title, input.author, input.genre)
}