use actix_web::{App, HttpServer};

mod handlers;
mod problem;

pub async fn launch_service() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handlers::health::status)
            .service(handlers::optimise::optimise_route)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
