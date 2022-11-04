use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{domain::member::OrganizationId, gateway::member::UserGateway, usecase};

#[derive(Deserialize)]
pub struct PathParameter {
    organization_id: String,
}

#[get("v1/users/{organization_id}")]
pub async fn get_users(path: web::Path<PathParameter>) -> impl Responder {
    let organization_id: i32 = path.organization_id.clone().parse().unwrap();
    let response = usecase::member::get_users(OrganizationId(organization_id), UserGateway).await;
    println!("{:?}", response);
    match response {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::Ok().body("Error"),
    }
    // HttpResponse::Ok().json(response)
}
