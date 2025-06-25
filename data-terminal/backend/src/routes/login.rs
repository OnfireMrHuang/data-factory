// src/v1/perm.rs
use axum::{http::StatusCode, routing::post, Json, Router};
use crate::models::web;

pub fn routes() -> Router {
    Router::new()
        .route("/", post(login))
}


async fn login(
    Json(payload): Json<web::LoginRequest>,
) -> (StatusCode, Json<web::Response<String>>) {
    let response = web::Response {
        result: true,
        msg: "Login successful".to_string(),
        data: "token".to_string(),
    };
    (StatusCode::OK, Json(response))
}