use actix_web::{get, HttpResponse, Responder};

use crate::usecase::user_usecase::get_user_usecase;

#[get("/user")]
async fn get_user() -> impl Responder {
    let result = get_user_usecase();
    HttpResponse::Ok().body(format!("{}", result.name))
}
