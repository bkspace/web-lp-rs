use crate::problem;
use actix_web::{get, web};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ResponseData {
    results: HashMap<String, f32>,
}

#[get("/api/optimise")]
pub async fn optimise_route() -> web::Json<ResponseData> {
    let result = problem::run_problem();
    web::Json(ResponseData { results: result.1 })
}
