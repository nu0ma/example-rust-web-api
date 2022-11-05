use actix_web::{get, post, web, HttpResponse, Responder};
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
        Err(_) => HttpResponse::Ok().body("Error"),
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

    println!("res {:?}", response);

    match response {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}
