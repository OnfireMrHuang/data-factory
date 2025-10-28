use crate::models::collection::*;
use crate::utils::error::RequestError;
use gloo::net::http::Request;

const API_BASE: &str = "http://localhost:3000/api/v1";

/// Fetch all collection tasks
pub async fn fetch_collection_tasks() -> Result<Vec<CollectTask>, RequestError> {
    let url = format!("{}/collections", API_BASE);

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| RequestError::network_error(e.to_string()))?;

    if !response.ok() {
        return Err(RequestError::http_error(response.status(), response.status_text()));
    }

    let tasks: Vec<CollectTask> = response
        .json()
        .await
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    Ok(tasks)
}

/// Fetch a specific collection task by ID
pub async fn fetch_collection_task_by_id(id: &str) -> Result<CollectTask, RequestError> {
    let url = format!("{}/collections/{}", API_BASE, id);

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| RequestError::network_error(e.to_string()))?;

    if !response.ok() {
        return Err(RequestError::http_error(response.status(), response.status_text()));
    }

    let task: CollectTask = response
        .json()
        .await
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    Ok(task)
}

/// Create a new collection task
pub async fn create_collection_task(
    request: CreateCollectTaskRequest,
) -> Result<CollectTask, RequestError> {
    let url = format!("{}/collections", API_BASE);

    let response = Request::post(&url)
        .json(&request)
        .map_err(|e| RequestError::serialization_error(e.to_string()))?
        .send()
        .await
        .map_err(|e| RequestError::network_error(e.to_string()))?;

    if !response.ok() {
        return Err(RequestError::http_error(response.status(), response.status_text()));
    }

    let task: CollectTask = response
        .json()
        .await
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    Ok(task)
}

/// Update an existing collection task
pub async fn update_collection_task(
    id: &str,
    request: UpdateCollectTaskRequest,
) -> Result<CollectTask, RequestError> {
    let url = format!("{}/collections/{}", API_BASE, id);

    let response = Request::put(&url)
        .json(&request)
        .map_err(|e| RequestError::serialization_error(e.to_string()))?
        .send()
        .await
        .map_err(|e| RequestError::network_error(e.to_string()))?;

    if !response.ok() {
        return Err(RequestError::http_error(response.status(), response.status_text()));
    }

    let task: CollectTask = response
        .json()
        .await
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    Ok(task)
}

/// Delete a collection task
pub async fn delete_collection_task(id: &str) -> Result<(), RequestError> {
    let url = format!("{}/collections/{}", API_BASE, id);

    let response = Request::delete(&url)
        .send()
        .await
        .map_err(|e| RequestError::network_error(e.to_string()))?;

    if !response.ok() {
        return Err(RequestError::http_error(response.status(), response.status_text()));
    }

    Ok(())
}

/// Apply a collection task to the data engine
pub async fn apply_collection_task(id: &str) -> Result<CollectTask, RequestError> {
    let url = format!("{}/collections/{}/apply", API_BASE, id);

    let response = Request::post(&url)
        .send()
        .await
        .map_err(|e| RequestError::network_error(e.to_string()))?;

    if !response.ok() {
        return Err(RequestError::http_error(response.status(), response.status_text()));
    }

    let task: CollectTask = response
        .json()
        .await
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    Ok(task)
}

/// Fetch tables from a datasource
pub async fn fetch_datasource_tables(datasource_id: &str) -> Result<Vec<TableMetadata>, RequestError> {
    let url = format!("{}/datasources/{}/tables", API_BASE, datasource_id);

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| RequestError::network_error(e.to_string()))?;

    if !response.ok() {
        return Err(RequestError::http_error(response.status(), response.status_text()));
    }

    let tables: Vec<TableMetadata> = response
        .json()
        .await
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    Ok(tables)
}

/// Fetch fields from a specific table
pub async fn fetch_table_fields(
    datasource_id: &str,
    table_name: &str,
) -> Result<Vec<FieldMetadata>, RequestError> {
    let url = format!(
        "{}/datasources/{}/tables/{}/fields",
        API_BASE, datasource_id, table_name
    );

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| RequestError::network_error(e.to_string()))?;

    if !response.ok() {
        return Err(RequestError::http_error(response.status(), response.status_text()));
    }

    let fields: Vec<FieldMetadata> = response
        .json()
        .await
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    Ok(fields)
}

/// Generate target schema from selected tables
pub async fn generate_target_schema(
    datasource_id: &str,
    resource_id: &str,
    selected_tables: Vec<TableSelection>,
) -> Result<TableSchema, RequestError> {
    let url = format!("{}/collections/generate-schema", API_BASE);

    let request_body = serde_json::json!({
        "datasource_id": datasource_id,
        "resource_id": resource_id,
        "selected_tables": selected_tables,
    });

    let response = Request::post(&url)
        .json(&request_body)
        .map_err(|e| RequestError::serialization_error(e.to_string()))?
        .send()
        .await
        .map_err(|e| RequestError::network_error(e.to_string()))?;

    if !response.ok() {
        return Err(RequestError::http_error(response.status(), response.status_text()));
    }

    #[derive(serde::Deserialize)]
    struct GenerateSchemaResponse {
        target_schema: TableSchema,
    }

    let result: GenerateSchemaResponse = response
        .json()
        .await
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    Ok(result.target_schema)
}
