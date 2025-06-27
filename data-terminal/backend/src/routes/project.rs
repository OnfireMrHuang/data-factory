use axum::{routing::{post, delete}, Router, http::StatusCode, Json, extract::Path};
use super::jwt::Claims;
use crate::models::web::Response;
use crate::models::project::Project;


pub fn routes() -> Router {
    Router::new()
    .route("/add", post(add_project))
    .route("/edit", post(edit_project))
    .route("/{code}", delete(del_project))
}


async fn add_project(
    claims: Claims,
    Json(payload): Json<Project>,
) -> (StatusCode, Json<Response<String>>) {
    (StatusCode::OK, Json(Response::<String>{
        result: true,
        msg: "".to_string(),
        data: "".to_string(),
    }))
}


async fn edit_project(
    claims: Claims,
    Json(payload): Json<Project>,
) -> (StatusCode, Json<Response<String>>) {
    (StatusCode::OK, Json(Response::<String>{
        result: true,
        msg: "".to_string(),
        data: "".to_string(),
    }))
}

async fn del_project(
    claims: Claims,
    Path(code): Path<String>,
) -> (StatusCode, Json<Response<String>>) {
    (StatusCode::OK, Json(Response::<String>{
        result: true,
        msg: "".to_string(),
        data: "".to_string(),
    }))
}