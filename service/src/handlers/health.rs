use actix_web::{get, HttpResponse, Responder};

#[get("/api/status")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("Ok!")
}
