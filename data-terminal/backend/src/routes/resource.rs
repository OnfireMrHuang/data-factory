use axum::{extract::{Path, Query}, http::StatusCode, routing::{delete, get, post}, Json, Router, debug_handler};
use super::jwt::Claims;
use crate::{autofac, models::resource::{Resource, ResourceReadOnly, ResourceCreateUpdate}};
use crate::models::web::{Response, PageQuery};

pub fn routes() -> Router {
    Router::new()
        .route("/add", post(add_resource))
        .route("/update", post(update_resource))
        .route("/list", get(list_resource))
        .route("/{id}", get(detail_resource))
        .route("/{id}", delete(delete_resource))
}

#[debug_handler]
async fn add_resource(
    claims: Claims,
    Json(resource): Json<ResourceCreateUpdate>,
) -> (StatusCode, Json<Response<String>>) {
    let result = autofac::get_global_app_state_ref().get_resource_service().add_resource(resource).await;
    match result {
        Ok(id) => (StatusCode::OK, Json(Response::success(id))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

#[debug_handler]
async fn list_resource(
    claims: Claims,
    Query(params): Query<PageQuery>,
) -> (StatusCode, Json<Response<Vec<ResourceReadOnly>>>) {
    let result = autofac::get_global_app_state_ref().get_resource_service().list_resource(params).await;
    match result {
        Ok(resources) => (StatusCode::OK, Json(Response::success(resources))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

#[debug_handler]
async fn delete_resource(
    claims: Claims,
    Path(id): Path<String>,
) -> (StatusCode, Json<Response<String>>) {
    let result = autofac::get_global_app_state_ref().get_resource_service().del_resource(id).await;
    match result {
        Ok(_) => (StatusCode::OK, Json(Response::success("".to_string()))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

#[debug_handler]
async fn update_resource(
    claims: Claims,
    Json(resource): Json<ResourceCreateUpdate>,
) -> (StatusCode, Json<Response<String>>) {
    let result = autofac::get_global_app_state_ref().get_resource_service().edit_resource(resource).await;
    match result {
        Ok(_) => (StatusCode::OK, Json(Response::success("".to_string()))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

#[debug_handler]
async fn detail_resource(
    claims: Claims,
    Path(id): Path<String>,
) -> (StatusCode, Json<Response<ResourceReadOnly>>) {
    let result = autofac::get_global_app_state_ref().get_resource_service().get_resource(id).await;
    match result {
        Ok(resource) => (StatusCode::OK, Json(Response::success(resource))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
} 