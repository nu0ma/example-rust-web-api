use actix_web::{App, HttpServer};

use app::utils::{log::set_log, set_db::set_db};
pub use user::rest::user::{add_user, delete_user, get_users, update_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_log();
    set_db().await;

    HttpServer::new(|| {
        App::new()
            .service(get_users)
            .service(add_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind(("0.0.0.0", 8085))?
    .run()
    .await
}
