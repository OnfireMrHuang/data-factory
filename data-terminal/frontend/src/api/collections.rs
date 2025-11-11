//! Collection task API operations

use crate::api::client::create_api_client;
use crate::models::collection::*;
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
    let mut builder = RequestBuilder::new().query_param("code", code);

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
/// Returns the task code on success
pub async fn create_collection_task(
    request: CreateCollectTaskRequest,
) -> Result<String, RequestError> {
    let client = create_api_client();

    let api_response: ApiResponse<String> = client
        .post_json("/api/v1/collection/add", None, request)
        .await?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Update an existing collection task
/// Returns the task code on success
pub async fn update_collection_task(
    code: &str,
    request: UpdateCollectTaskRequest,
) -> Result<String, RequestError> {
    let client = create_api_client();

    // Build the request body with code included
    let body = serde_json::json!({
        "code": code,
        "name": request.name,
        "description": request.description,
        "rule": request.rule,
    });

    let api_response: ApiResponse<String> = client
        .post_json("/api/v1/collection/update", None, body)
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
pub async fn apply_collection_task(code: &str) -> Result<String, RequestError> {
    let client = create_api_client();

    let api_response: ApiResponse<String> = client
        .post_json(&format!("/api/v1/collection/{}/apply", code), None, serde_json::json!({}))
        .await?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Fetch tables from a datasource
pub async fn fetch_datasource_tables(datasource_id: &str) -> Result<Vec<TableMetadata>, RequestError> {
    let client = create_api_client();

    let api_response: ApiResponse<Vec<TableMetadata>> = client
        .get_json(&format!("/api/v1/datasources/{}/tables", datasource_id), None)
        .await?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Fetch fields from a specific table
pub async fn fetch_table_fields(
    datasource_id: &str,
    table_name: &str,
) -> Result<Vec<FieldMetadata>, RequestError> {
    let client = create_api_client();

    let api_response: ApiResponse<Vec<FieldMetadata>> = client
        .get_json(
            &format!("/api/v1/datasources/{}/tables/{}/fields", datasource_id, table_name),
            None,
        )
        .await?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Generate target schema from selected tables
pub async fn generate_target_schema(
    datasource_id: &str,
    resource_id: &str,
    selected_tables: Vec<TableSelection>,
) -> Result<TableSchema, RequestError> {
    let client = create_api_client();

    let request_body = serde_json::json!({
        "datasource_id": datasource_id,
        "resource_id": resource_id,
        "selected_tables": selected_tables,
    });

    #[derive(serde::Deserialize)]
    struct GenerateSchemaResponse {
        target_schema: TableSchema,
    }

    let api_response: ApiResponse<GenerateSchemaResponse> = client
        .post_json("/api/v1/collections/generate-schema", None, request_body)
        .await?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data.target_schema)
}
