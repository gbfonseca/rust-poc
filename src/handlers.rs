use actix_web::{web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]

pub struct Health {
    message: String,
    status: i32
}


pub async fn health() -> Result<impl Responder> {
    let health = Health{
      message: "Everything is OK!".to_string(),
      status: 200
    };

  Ok(web::Json(health))
}