use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
    routing::{get, post, delete},
    Router, debug_handler,
};
use serde::{Deserialize, Serialize};
use super::jwt::Claims;
use crate::{autofac, models::collection::*, models::web::Response, models::test_run::*};

/// Register collection routes
pub fn routes() -> Router {
    Router::new()
        .route("/list", get(list_collection_tasks))
        .route("/add", post(create_collection_task))
        .route("/detail", get(get_collection_task))
        .route("/update", post(update_collection_task))
        .route("/{code}", delete(delete_collection_task))
        .route("/{code}/apply", post(apply_collection_task))
        .route("/{code}/test_run", post(execute_test_run))
        .route("/{code}/test_run_status", get(get_test_run_status))
        .route("/{code}/test_run_logs", get(get_test_run_logs))
        .route("/{code}/test_run_preview", get(get_test_run_preview))
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
    Json(request): Json<CreateOrUpdateCollectTaskRequest>,
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


#[debug_handler]
async fn get_collection_task(
    claims: Claims,
    Query(params): Query<DetailRequest>,
) -> (StatusCode, Json<Response<CollectTaskReadOnly>>) {
    let result = autofac::get_global_app_state_ref()
        .get_collection_service()
        .get_task(claims.project, params)
        .await;

    match result {
        Ok(Some(task)) => (StatusCode::OK, Json(Response::success(task))),
        Ok(None) => (StatusCode::OK, Json(Response::error(format!("Task not found")))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

/// POST /api/v1/collections/update - Update collection task
#[debug_handler]
async fn update_collection_task(
    claims: Claims,
    Json(request): Json<CreateOrUpdateCollectTaskRequest>,
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
    Path(code): Path<String>,
) -> (StatusCode, Json<Response<CollectTaskReadOnly>>) {
    let result = autofac::get_global_app_state_ref()
        .get_collection_service()
        .apply_task(claims.project, &code)
        .await;

    match result {
        Ok(task) => (StatusCode::OK, Json(Response::success(task))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

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
    Path(code): Path<String>,
) -> (StatusCode, Json<Response<String>>) {
    let result = autofac::get_global_app_state_ref()
        .get_collection_service()
        .delete_task(claims.project, &code)
        .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(Response::success("".to_string()))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

// ============================================================================
// Test Run Routes
// ============================================================================

/// POST /api/v1/collection/{code}/test_run - Execute test run
#[debug_handler]
async fn execute_test_run(
    claims: Claims,
    Path(code): Path<String>,
) -> (StatusCode, Json<Response<TestRunTaskIdResponse>>) {
    let result = autofac::get_global_app_state_ref()
        .get_collection_service()
        .execute_test_run(claims.project, &code)
        .await;

    match result {
        Ok(task_id) => (StatusCode::OK, Json(Response::success(TestRunTaskIdResponse { task_id }))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

/// Query parameters for test run status/logs/preview
#[derive(Deserialize)]
pub struct TestRunQuery {
    pub task_id: String,
    pub start_timestamp: Option<i64>,
}

/// GET /api/v1/collection/{code}/test_run_status - Get test run status
#[debug_handler]
async fn get_test_run_status(
    claims: Claims,
    Path(code): Path<String>,
    Query(query): Query<TestRunQuery>,
) -> (StatusCode, Json<Response<TestRunStatusResponse>>) {
    let result = autofac::get_global_app_state_ref()
        .get_collection_service()
        .get_test_run_status(claims.project, &query.task_id)
        .await;

    match result {
        Ok(status) => (StatusCode::OK, Json(Response::success(status))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

/// GET /api/v1/collection/{code}/test_run_logs - Get test run logs
#[debug_handler]
async fn get_test_run_logs(
    claims: Claims,
    Path(code): Path<String>,
    Query(query): Query<TestRunQuery>,
) -> (StatusCode, Json<Response<TestLogResponse>>) {
    let result = autofac::get_global_app_state_ref()
        .get_collection_service()
        .get_test_run_logs(claims.project, &query.task_id, query.start_timestamp)
        .await;

    match result {
        Ok(logs) => (StatusCode::OK, Json(Response::success(logs))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}

/// GET /api/v1/collection/{code}/test_run_preview - Get test run data preview
#[debug_handler]
async fn get_test_run_preview(
    claims: Claims,
    Path(code): Path<String>,
    Query(query): Query<TestRunQuery>,
) -> (StatusCode, Json<Response<TablePreviewResponse>>) {
    let result = autofac::get_global_app_state_ref()
        .get_collection_service()
        .get_test_run_preview(claims.project, &query.task_id)
        .await;

    match result {
        Ok(preview) => (StatusCode::OK, Json(Response::success(preview))),
        Err(e) => (StatusCode::OK, Json(Response::error(e.to_string()))),
    }
}
