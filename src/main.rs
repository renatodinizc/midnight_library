use std::net::TcpListener;

use bookstore_api::{configuration::get_configuration, run};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let tcp_listener = TcpListener::bind("localhost:8080").expect("Failed to bind random port");
    let config = get_configuration().expect("Failed to read configuration.");

    let db_pool = PgPool::connect(config.database.database_url().as_str())
        .await
        .expect("Failed to connect to Postgres.");

    run(tcp_listener, db_pool)?.await
}
