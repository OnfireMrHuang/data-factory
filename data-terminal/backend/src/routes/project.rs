use axum::{extract::{Path, Query}, http::StatusCode, response::IntoResponse, routing::{delete, get, post}, Json, Router};
use super::jwt::Claims;
use crate::{autofac, models::project::Project};
use crate::models::web::{Response, PageQuery};
use crate::models::error::Error;


pub fn routes() -> Router {
    Router::new()
        .route("/add", post(add_project))
        .route("/list", get(list_project))
        .route("/update", post(update_project))
        .route("/delete", delete(delete_project))
        .route("/detail", get(detail_project))
}


async fn add_project(
    claims: Claims,
    Json(project): Json<Project>,
) -> (StatusCode, Json<Response<String>>) {
    (StatusCode::OK, Json(Response::success("success".to_string())))
}

async fn list_project(
    claims: Claims,
    Query(params): Query<PageQuery>,
) -> (StatusCode, Json<Response<Vec<Project>>>) {
    (StatusCode::OK, Json(Response::success(vec![])))
}

async fn delete_project(
    claims: Claims,
    Path(code): Path<String>,
) -> (StatusCode, Json<Response<String>>) {
    (StatusCode::OK, Json(Response::success("success".to_string())))
}

async fn update_project(
    claims: Claims,
    Json(project): Json<Project>,
) -> (StatusCode, Json<Response<String>>) {
    (StatusCode::OK, Json(Response::success("success".to_string())))
}

async fn detail_project(
    claims: Claims,
    Path(code): Path<String>,
) -> (StatusCode, Json<Response<Project>>) {
    (StatusCode::OK, Json(Response::success(Project::default())))
}



