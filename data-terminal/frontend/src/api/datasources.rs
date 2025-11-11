//! Datasource API operations

use crate::api::client::create_api_client;
use crate::models::datasource::{DataSource, DataSourceCreateUpdate};
use crate::utils::error::RequestError;
use crate::utils::request::{HttpRequest, RequestBuilder};
use serde::Deserialize;
use crate::models::protocol::ApiResponse;


/// Fetch all datasources with pagination
pub async fn fetch_datasources(page: u32, page_size: u32) -> Result<Vec<DataSource>, RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .query_param("page", page)
        .query_param("page_size", page_size)
        .build();

    let response_text = client
        .get("/api/v1/datasource/list", Some(req_config))
        .await?;

    let api_response: ApiResponse<Vec<DataSource>> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Fetch a specific datasource by ID
pub async fn fetch_datasource_by_id(id: &str) -> Result<DataSource, RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .build();

    let response_text = client
        .get(&format!("/api/v1/datasource/{}", id), Some(req_config))
        .await?;

    let api_response: ApiResponse<DataSource> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Create a new datasource
/// Returns the datasource ID on success
pub async fn create_datasource(datasource: DataSourceCreateUpdate) -> Result<String, RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .build();

    let response_text = client
        .post("/api/v1/datasource/add", Some(req_config), datasource)
        .await?;

    let api_response: ApiResponse<String> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Update an existing datasource
/// Returns the datasource ID on success
pub async fn update_datasource(datasource: DataSourceCreateUpdate) -> Result<String, RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .build();

    let response_text = client
        .post("/api/v1/datasource/update", Some(req_config), datasource)
        .await?;

    let api_response: ApiResponse<String> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Delete a datasource by ID
pub async fn delete_datasource(id: &str) -> Result<(), RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .build();

    let response_text = client
        .delete(&format!("/api/v1/datasource/{}", id), Some(req_config))
        .await?;

    let api_response: ApiResponse<String> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(())
}

/// Test datasource connection (ping)
/// Returns Ok(()) if connection is successful
pub async fn test_datasource_connection(datasource: DataSourceCreateUpdate) -> Result<String, RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .build();

    let response_text = client
        .post("/api/v1/datasource/ping", Some(req_config), datasource)
        .await?;

    let api_response: ApiResponse<String> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}
