//! Resource API operations

use crate::api::client::create_api_client;
use crate::models::resource::{Resource, ResourceCreateUpdate, ResourceFormData};
use crate::utils::error::RequestError;
use crate::utils::request::{HttpRequest, RequestBuilder};
use crate::models::protocol::ApiResponse;

/// Fetch all resources
pub async fn fetch_resources() -> Result<Vec<Resource>, RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .build();

    let response_text = client
        .get("/api/v1/resource/list", Some(req_config))
        .await?;

    let api_response: ApiResponse<Vec<Resource>> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Fetch a specific resource by ID
pub async fn fetch_resource_by_id(id: &str) -> Result<Resource, RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .build();

    let response_text = client
        .get(&format!("/api/v1/resource/{}", id), Some(req_config))
        .await?;

    let api_response: ApiResponse<Resource> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Create a new resource
/// Returns the resource ID on success
pub async fn create_resource(resource: ResourceFormData) -> Result<String, RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .build();

    let response_text = client
        .post("/api/v1/resource/add", Some(req_config), resource)
        .await?;

    let api_response: ApiResponse<String> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Update an existing resource
/// Returns the resource ID on success
pub async fn update_resource(resource: ResourceCreateUpdate) -> Result<String, RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .build();

    let response_text = client
        .post("/api/v1/resource/update", Some(req_config), resource)
        .await?;

    let api_response: ApiResponse<String> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Delete a resource by ID
pub async fn delete_resource(id: &str) -> Result<(), RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .build();

    let response_text = client
        .delete(&format!("/api/v1/resource/{}", id), Some(req_config))
        .await?;

    let api_response: ApiResponse<String> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(())
}
