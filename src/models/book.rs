use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author_id: Uuid,
    pub genre: String,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateBookData {
    pub title: String,
    pub author: String,
    pub genre: String,
}

#[derive(Deserialize)]
pub struct BookId {
    pub id: String,
}
