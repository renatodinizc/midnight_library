pub mod routes;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(address: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/books", web::get().to(routes::books))
            .route("/books/create", web::post().to(routes::create_book))
    })
    .listen(address)?
    .run();

    Ok(server)
}
