use actix_web::{App, HttpServer, web};
mod handlers;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(handlers::health))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}