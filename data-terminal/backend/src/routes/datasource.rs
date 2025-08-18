
use axum::{extract::{Path, Query}, http::StatusCode, routing::{delete, get, post}, Json, Router, debug_handler};
use super::jwt::Claims;
use crate::{autofac, models::datasource::{DataSourceReadOnly, DataSourceCreateUpdate}};
use crate::models::web::{Response, PageQuery};

pub fn routes() -> Router {
    Router::new()
        .route("/add", post(add_datasource))
        .route("/update", post(update_datasource))
        .route("/list", get(list_datasource))
        .route("/{id}", get(detail_datasource))
        .route("/{id}", delete(delete_datasource))
}

#[debug_handler]
async fn add_datasource(
    claims: Claims,
    Json(datasource): Json<DataSourceCreateUpdate>,
) -> (StatusCode, Json<Response<String>>) {
    let result = autofac::get_global_app_state_ref().get_datasource_service().add_datasource(claims.project, datasource).await;
    match result {
        Ok(id) => (StatusCode::OK, Json(Response::success(id))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

#[debug_handler]
async fn list_datasource(
    claims: Claims,
    Query(params): Query<PageQuery>,
) -> (StatusCode, Json<Response<Vec<DataSourceReadOnly>>>) {
    let result = autofac::get_global_app_state_ref().get_datasource_service().list_datasource(claims.project, params).await;
    match result {
        Ok(datasources) => (StatusCode::OK, Json(Response::success(datasources))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

#[debug_handler]
async fn delete_datasource(
    claims: Claims,
    Path(id): Path<String>,
) -> (StatusCode, Json<Response<String>>) {
    let result = autofac::get_global_app_state_ref().get_datasource_service().del_datasource(claims.project, id).await;
    match result {
        Ok(_) => (StatusCode::OK, Json(Response::success("".to_string()))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

#[debug_handler]
async fn update_datasource(
    claims: Claims,
    Json(datasource): Json<DataSourceCreateUpdate>,
) -> (StatusCode, Json<Response<String>>) {
    let result = autofac::get_global_app_state_ref().get_datasource_service().edit_datasource(claims.project, datasource).await;
    match result {
        Ok(_) => (StatusCode::OK, Json(Response::success("".to_string()))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

#[debug_handler]
async fn detail_datasource(
    claims: Claims,
    Path(id): Path<String>,
) -> (StatusCode, Json<Response<DataSourceReadOnly>>) {
    let result = autofac::get_global_app_state_ref().get_datasource_service().get_datasource(claims.project, id).await;
    match result {
        Ok(datasource) => (StatusCode::OK, Json(Response::success(datasource))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

