use actix_web::{App, HttpServer};
pub mod routes;
pub mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
