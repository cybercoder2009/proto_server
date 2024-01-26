use actix_web::{web,post,Responder,HttpResponse};
use serde::{Deserialize,Serialize};
use serde_json::json;
use mongodb::bson::doc;
use crate::s_state::State;

pub async fn test(
    _state: web::Data<State>,
) -> impl Responder {
    println!("test");
    HttpResponse::Ok().json(json!({
        "msg": "hello world"
    }))
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Register {
    pub vk: String,
}

#[post("/register")]
pub async fn register(
    _state: web::Data<State>,
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

#[post("/login")]
pub async fn login(
    json: web::Json<Login>,
) -> impl Responder {
    println!("login {:?}", json);
    HttpResponse::Ok().json(json)
}