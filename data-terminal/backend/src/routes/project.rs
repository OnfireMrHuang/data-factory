use axum::{extract::{Path, Query}, http::StatusCode, routing::{delete, get, post}, Json, Router, debug_handler};
use super::jwt::Claims;
use crate::{autofac, models::project::Project};
use crate::models::web::{Response, PageQuery};


pub fn routes() -> Router {
    Router::new()
        .route("/add", post(add_project))
        .route("/update", post(update_project))
        .route("/list", get(list_project))
        .route("/{code}", get(detail_project))
        .route("/{code}", delete(delete_project))
}


#[debug_handler]
async fn add_project(
    claims: Claims,
    Json(project): Json<Project>,
) -> (StatusCode, Json<Response<String>>) {
    let result = autofac::get_global_app_state_ref().get_project_service().add_project(project).await;
    match result {
        Ok(code) => (StatusCode::OK, Json(Response::success(code))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

#[debug_handler]
async fn list_project(
    claims: Claims,
    Query(params): Query<PageQuery>,
) -> (StatusCode, Json<Response<Vec<Project>>>) {
    let result = autofac::get_global_app_state_ref().get_project_service().list_project(params).await;
    match result {
        Ok(projects) => (StatusCode::OK, Json(Response::success(projects))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

#[debug_handler]
async fn delete_project(
    claims: Claims,
    Path(code): Path<String>,
) -> (StatusCode, Json<Response<String>>) {
  let result = autofac::get_global_app_state_ref().get_project_service().del_project(code).await;
  match result {
    Ok(_) => (StatusCode::OK, Json(Response::success("".to_string()))),
    Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
  }
}

#[debug_handler]
async fn update_project(
    claims: Claims,
    Json(project): Json<Project>,
) -> (StatusCode, Json<Response<String>>) {
  let result = autofac::get_global_app_state_ref().get_project_service().edit_project(project).await;
  match result {
    Ok(_) => (StatusCode::OK, Json(Response::success("".to_string()))),
    Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
  }
}

#[debug_handler]
async fn detail_project(
    claims: Claims,
    Path(code): Path<String>,
) -> (StatusCode, Json<Response<Project>>) {
  let result = autofac::get_global_app_state_ref().get_project_service().get_project(code).await;
  match result {
    Ok(project) => (StatusCode::OK, Json(Response::success(project))),
    Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
  }
}



