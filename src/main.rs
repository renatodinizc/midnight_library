use std::net::TcpListener;

use bookstore_api::run;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let tcp_listener = TcpListener::bind("localhost:8080").expect("Failed to bind random port");
    let db_pool = PgPool::connect("postgres://postgres:password@localhost:5432/bookstore_api")
        .await
        .expect("Failed to connect to Postgres.");

    run(tcp_listener, db_pool)?.await
}
