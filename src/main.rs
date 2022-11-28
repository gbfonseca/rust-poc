use actix_web::{App, HttpServer};
mod controllers;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(controllers::health_controller)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}