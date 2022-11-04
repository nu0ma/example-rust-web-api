use actix_web::{App, HttpServer};
use app::{
    rest::member::get_users,
    utils::{log::set_log, set_db::set_db},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_log();
    set_db().await;

    HttpServer::new(|| App::new().service(get_users))
        .bind(("0.0.0.0", 8085))?
        .run()
        .await
}
