//! Collection task API operations

use crate::api::client::create_api_client;
use crate::models::collection::*;
use crate::models::test_run::*;
use crate::utils::error::RequestError;
use crate::utils::request::{HttpRequest, RequestBuilder};
use serde::Deserialize;
use crate::models::protocol::ApiResponse;

/// Paginated list response
#[derive(Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
    pub total: u32,
}

/// Fetch all collection tasks with optional filters
pub async fn fetch_collection_tasks(
    page: Option<u32>,
    page_size: Option<u32>,
    stage: Option<&str>,
    category: Option<&str>,
    collect_type: Option<&str>,
) -> Result<PaginatedResponse<CollectTask>, RequestError> {
    let client = create_api_client();
    let mut builder = RequestBuilder::new();

    builder = builder.header("Content-Type", "application/json");
    if let Some(p) = page {
        builder = builder.query_param("page", p);
    }
    if let Some(ps) = page_size {
        builder = builder.query_param("page_size", ps);
    }
    if let Some(s) = stage {
        builder = builder.query_param("stage", s);
    }
    if let Some(c) = category {
        builder = builder.query_param("category", c);
    }
    if let Some(ct) = collect_type {
        builder = builder.query_param("collect_type", ct);
    }

    let config = builder.build();
    let api_response: ApiResponse<PaginatedResponse<CollectTask>> = client
        .get_json("/api/v1/collection/list", Some(config))
        .await?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Fetch a specific collection task by code
pub async fn fetch_collection_task_by_code(
    code: &str,
    stage: Option<&str>,
) -> Result<CollectTask, RequestError> {
    let client = create_api_client();
    let mut builder = RequestBuilder::new().header("Content-Type", "application/json").query_param("code", code);

    if let Some(s) = stage {
        builder = builder.query_param("stage", s);
    }

    let config = builder.build();
    let api_response: ApiResponse<CollectTask> = client
        .get_json("/api/v1/collection/detail", Some(config))
        .await?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Create a new collection task
/// Returns the created task on success
pub async fn create_collection_task(
    request: CreateOrUpdateCollectTaskRequest,
) -> Result<CollectTask, RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
    .header("Content-Type", "application/json")
    .build();

    let api_response: ApiResponse<CollectTask> = client
        .post_json("/api/v1/collection/add", Some(req_config), request)
        .await?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Update an existing collection task
/// Returns the updated task on success
pub async fn update_collection_task(
    request: CreateOrUpdateCollectTaskRequest,
) -> Result<CollectTask, RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
    .header("Content-Type", "application/json")
    .build();

    let api_response: ApiResponse<CollectTask> = client
        .post_json("/api/v1/collection/update", Some(req_config), request)
        .await?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Delete a collection task
pub async fn delete_collection_task(code: &str) -> Result<(), RequestError> {
    let client = create_api_client();

    let response_text: String = client
        .delete(&format!("/api/v1/collection/{}", code), None)
        .await?;

    let api_response: ApiResponse<String> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(())
}

/// Apply a collection task to the data engine
pub async fn apply_collection_task(code: &str) -> Result<CollectTask, RequestError> {
    let client = create_api_client();

    let api_response: ApiResponse<CollectTask> = client
        .post_json(&format!("/api/v1/collection/{}/apply", code), None, serde_json::json!({}))
        .await?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

// ============================================================================
// Test Run API Operations
// ============================================================================

/// Execute a test run for a collection task
pub async fn execute_test_run(code: &str) -> Result<TestRunTaskIdResponse, RequestError> {
    let client = create_api_client();

    let api_response: ApiResponse<TestRunTaskIdResponse> = client
        .post_json(&format!("/api/v1/collection/{}/test_run", code), None, serde_json::json!({}))
        .await?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Get test run status with steps
pub async fn get_test_run_status(code: &str, task_id: &str) -> Result<TestRunStatusResponse, RequestError> {
    let client = create_api_client();

    let config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .query_param("task_id", task_id)
        .build();

    let api_response: ApiResponse<TestRunStatusResponse> = client
        .get_json(&format!("/api/v1/collection/{}/test_run_status", code), Some(config))
        .await?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Get test run logs
pub async fn get_test_run_logs(
    code: &str,
    task_id: &str,
    start_timestamp: Option<i64>,
) -> Result<TestLogResponse, RequestError> {
    let client = create_api_client();

    let mut builder = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .query_param("task_id", task_id);

    if let Some(ts) = start_timestamp {
        builder = builder.query_param("start_timestamp", ts);
    }

    let config = builder.build();

    let api_response: ApiResponse<TestLogResponse> = client
        .get_json(&format!("/api/v1/collection/{}/test_run_logs", code), Some(config))
        .await?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Get test run data preview
pub async fn get_test_run_preview(code: &str, task_id: &str) -> Result<TablePreviewResponse, RequestError> {
    let client = create_api_client();

    let config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .query_param("task_id", task_id)
        .build();

    let api_response: ApiResponse<TablePreviewResponse> = client
        .get_json(&format!("/api/v1/collection/{}/test_run_preview", code), Some(config))
        .await?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}
