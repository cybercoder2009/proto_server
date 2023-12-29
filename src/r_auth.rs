use actix_web::{web,Responder,HttpResponse};
use serde::{Deserialize,Serialize};
use mongodb::bson::doc;
use crate::s_state::State;

#[derive(Debug,Serialize,Deserialize)]
pub struct Register {
    pub vk: String,
}

pub async fn register(
    state: web::Data<State>,
    json: web::Json<Register>,
) -> impl Responder {
    println!("register {:?}", json);
    HttpResponse::Ok().json(json)
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

pub async fn login(
    json: web::Json<Login>,
) -> impl Responder {
    println!("login {:?}", json);
    HttpResponse::Ok().json(json)
}