use serde::Serialize;

#[derive(Serialize)]
pub struct Health {
    message: String,
    status: i32
}

pub fn health() -> Health {
  let health = Health{
    message: "Everything is OK!".to_string(),
    status: 200
  };

  return health;
}