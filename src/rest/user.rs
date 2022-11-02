use actix_web::{get, HttpResponse, Responder};

use crate::{domain::user::Id, usecase};

#[get("v1/users")]
pub async fn get_user() -> impl Responder {
    // let response = usecase::user::get_users(todo!(), todo!());
    HttpResponse::Ok()
}
