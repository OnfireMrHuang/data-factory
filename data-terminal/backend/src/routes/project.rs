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
) -> impl IntoResponse {
    let app_state = autofac::get_global_app_state_ref();
    let project_service = app_state.get_project_service();
    let result: Result<String, Error> = project_service.add_project(project).await;
    match result {
        Ok(project) => (StatusCode::OK, Json(Response::success(project))).into_response(),
        Err(e) => (StatusCode::OK, Json(Response::<String>::error(e.to_string()))).into_response(),
    }
}

async fn list_project(
    claims: Claims,
    Query(params): Query<PageQuery>,
) -> impl IntoResponse {
    Err::<Json<Response<String>>, Error>(Error::NotImplemented)
}

async fn delete_project(
    claims: Claims,
    Path(code): Path<String>,
) -> impl IntoResponse {
    Err::<Json<Response<String>>, Error>(Error::NotImplemented)
}

async fn update_project(
    claims: Claims,
    Json(project): Json<Project>,
) -> impl IntoResponse {
    Err::<Json<Response<String>>, Error>(Error::NotImplemented)
}

async fn detail_project(
    claims: Claims,
    Path(code): Path<String>,
) -> impl IntoResponse {
    Err::<Json<Response<String>>, Error>(Error::NotImplemented)
}



