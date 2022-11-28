use actix_web::{ Responder, Result, get};
use actix_web::web;

use self::health::health;

mod health;

#[get("/health")]
pub async fn health_controller() -> Result<impl Responder> {
  Ok(web::Json(health()))
}