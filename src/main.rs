use std::net::TcpListener;

use bookstore_api::{configuration::get_configuration, startup::run};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration.");

    let tcp_listener =
        TcpListener::bind(config.server_address).expect("Failed to bind random port");

    let db_pool = PgPool::connect(config.database.database_url().as_str())
        .await
        .expect("Failed to connect to Postgres.");

    run(tcp_listener, db_pool)?.await
}
