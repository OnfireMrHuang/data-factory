use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
    routing::{get, post, delete},
    Router, debug_handler,
};
use serde::{Deserialize, Serialize};
use super::jwt::Claims;
use crate::{autofac, models::collection::*, models::web::Response};

/// Register collection routes
pub fn routes() -> Router {
    Router::new()
        .route("/list", get(list_collection_tasks))
        .route("/add", post(create_collection_task))
        .route("/{code}", get(get_collection_task))
        .route("/update", post(update_collection_task))
        .route("/{code}", delete(delete_collection_task))
        .route("/{code}/apply", post(apply_collection_task))
}

/// Query parameters for list endpoint
#[derive(Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub stage: Option<TaskStage>,
    pub category: Option<CollectionCategory>,
    pub collect_type: Option<CollectType>,
}

/// Pagination response wrapper
#[derive(Serialize, Default)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Serialize, Default)]
pub struct Pagination {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
}

/// POST /api/v1/collections/add - Create collection task
#[debug_handler]
async fn create_collection_task(
    claims: Claims,
    Json(request): Json<CreateCollectTaskRequest>,
) -> (StatusCode, Json<Response<CollectTaskReadOnly>>) {
    let result = autofac::get_global_app_state_ref()
        .get_collection_service()
        .create_task(claims.project, request)
        .await;

    match result {
        Ok(task) => (StatusCode::OK, Json(Response::success(task))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

/// GET /api/v1/collections/:id - Get collection task by ID
#[debug_handler]
async fn get_collection_task(
    claims: Claims,
    Path(id): Path<String>,
) -> (StatusCode, Json<Response<CollectTaskReadOnly>>) {
    let result = autofac::get_global_app_state_ref()
        .get_collection_service()
        .get_task(claims.project, &id)
        .await;

    match result {
        Ok(Some(task)) => (StatusCode::OK, Json(Response::success(task))),
        Ok(None) => (StatusCode::OK, Json(Response::error(format!("Task {} not found", id)))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

/// POST /api/v1/collections/update - Update collection task
#[debug_handler]
async fn update_collection_task(
    claims: Claims,
    Json(request): Json<UpdateCollectTaskRequest>,
) -> (StatusCode, Json<Response<CollectTaskReadOnly>>) {
    let result = autofac::get_global_app_state_ref()
        .get_collection_service()
        .update_task(claims.project, request)
        .await;

    match result {
        Ok(task) => (StatusCode::OK, Json(Response::success(task))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

/// POST /api/v1/collections/:id/apply - Apply collection task
#[debug_handler]
async fn apply_collection_task(
    claims: Claims,
    Path(id): Path<String>,
) -> (StatusCode, Json<Response<CollectTaskReadOnly>>) {
    let result = autofac::get_global_app_state_ref()
        .get_collection_service()
        .apply_task(claims.project, &id)
        .await;

    match result {
        Ok(task) => (StatusCode::OK, Json(Response::success(task))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

/// GET /api/v1/collections/list - List all collection tasks
#[debug_handler]
async fn list_collection_tasks(
    claims: Claims,
    Query(query): Query<ListQuery>,
) -> (StatusCode, Json<Response<PaginatedResponse<CollectTaskReadOnly>>>) {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20).min(100);

    let result = autofac::get_global_app_state_ref()
        .get_collection_service()
        .list_tasks(
            claims.project,
            page,
            page_size,
            query.stage,
            query.category,
            query.collect_type,
        )
        .await;

    match result {
        Ok((tasks, total)) => {
            let data = PaginatedResponse {
                data: tasks,
                pagination: Pagination { page, page_size, total },
            };
            (StatusCode::OK, Json(Response::success(data)))
        }
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

/// DELETE /api/v1/collections/:id - Delete collection task
#[debug_handler]
async fn delete_collection_task(
    claims: Claims,
    Path(id): Path<String>,
) -> (StatusCode, Json<Response<String>>) {
    let result = autofac::get_global_app_state_ref()
        .get_collection_service()
        .delete_task(claims.project, &id)
        .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(Response::success("".to_string()))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}
