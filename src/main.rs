use bookstore_api::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
