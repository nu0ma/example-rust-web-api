use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{
    domain::user::OrganizationId,
    gateway::user::UserGateway,
    usecase::{self},
};

#[derive(Deserialize)]
pub struct PathParameter {
    organization_id: String,
}

#[get("v1/users/{organization_id}")]
pub async fn get_users(path: web::Path<PathParameter>) -> impl Responder {
    let organization_id: i32 = path.organization_id.clone().parse().unwrap();
    let response = usecase::user::get_users(OrganizationId(organization_id), UserGateway).await;
    match response {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize, Clone)]
pub struct Info {
    name: String,
    organization_name: String,
}

#[post("v1/user")]
pub async fn add_user(info: web::Json<Info>) -> impl Responder {
    let response = usecase::user::add_user(
        info.name.clone(),
        info.organization_name.clone(),
        UserGateway,
    )
    .await;

    match response {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize, Clone)]
pub struct NewName {
    name: String,
}

#[put("v1/user/{id}")]
pub async fn update_user(id: web::Path<i32>, new_name: web::Json<NewName>) -> impl Responder {
    let response = usecase::user::update_user(id.clone(), new_name.name.clone(), UserGateway).await;
    match response {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Deserialize, Clone)]
struct UserId {
    id: i32,
}
#[delete("v1/user/{id}")]
pub async fn delete_user(id: web::Path<UserId>) -> impl Responder {
    let response = usecase::user::delete_user(id.id, UserGateway).await;

    match response {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
