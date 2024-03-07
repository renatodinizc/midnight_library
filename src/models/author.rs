use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub id: Uuid,
    pub name: String,
    pub nationality: String,
    pub created_at: DateTime<Local>,
}

#[derive(Deserialize)]
pub struct AuthorId {
    pub id: String,
}
