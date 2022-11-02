use actix_web::{App, HttpServer};
use rest::user::get_user;

mod domain;
mod driver;
mod gateway;
mod port;
mod rest;
mod usecase;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_user))
        .bind(("127.0.0.1", 8085))?
        .run()
        .await
}
