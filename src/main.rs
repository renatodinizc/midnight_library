use std::net::TcpListener;

use bookstore_api::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let tcp_listener = TcpListener::bind("localhost:8080").expect("Failed to bind random port");
    run(tcp_listener)?.await
}
