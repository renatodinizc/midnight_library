use crate::validations::user::NewUser;
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct NewUserData {
    pub name: String,
    pub email: String,
}

pub async fn create_user(input: Json<NewUserData>, db_pool: Data<PgPool>) -> HttpResponse {
    let new_user: NewUser = match input.0.try_into() {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error),
    };

    match sqlx::query!(
        "INSERT INTO users (name, email, created_at)
        VALUES ($1, $2, $3)
        RETURNING id",
        new_user.name.as_ref(),
        new_user.email.as_ref(),
        Utc::now()
    )
    .fetch_one(db_pool.get_ref())
    .await
    {
        Ok(record) => HttpResponse::Ok().json(json!({
            "message": "User created successfully!",
            "user_id": record.id
        })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
