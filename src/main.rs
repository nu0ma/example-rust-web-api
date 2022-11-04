use actix_web::{App, HttpServer};
use app::rest::member::get_users;
use tracing::Level;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    HttpServer::new(|| App::new().service(get_users))
        .bind(("0.0.0.0", 8085))?
        .run()
        .await
}
