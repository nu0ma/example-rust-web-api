use actix_web::{App, HttpServer};
use rest::user::get_users;
use tracing::Level;

mod domain;
mod driver;
mod gateway;
mod port;
mod rest;
mod usecase;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    HttpServer::new(|| App::new().service(get_users))
        .bind(("0.0.0.0", 8085))?
        .run()
        .await
}
