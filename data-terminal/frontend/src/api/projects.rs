//! Project API operations

use crate::api::client::create_api_client;
use crate::models::project::Project;
use crate::utils::error::RequestError;
use crate::utils::request::HttpRequest;
use crate::utils::request::RequestBuilder;
use serde::Deserialize;
use crate::models::protocol::ApiResponse;

/// Fetch all projects
pub async fn fetch_projects() -> Result<Vec<Project>, RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .build();

    let response_text = client
        .get("/api/v1/project/list", Some(req_config))
        .await?;

    let api_response: ApiResponse<Vec<Project>> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Create a new project
/// Returns the project code on success
pub async fn create_project(project: Project) -> Result<String, RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .build();

    let response_text = client
        .post("/api/v1/project/add", Some(req_config), project)
        .await?;

    let api_response: ApiResponse<String> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Update an existing project
/// Returns the project code on success
pub async fn update_project(project: Project) -> Result<String, RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .build();

    let response_text = client
        .post("/api/v1/project/update", Some(req_config), project)
        .await?;

    let api_response: ApiResponse<String> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(api_response.data)
}

/// Delete a project by code
pub async fn delete_project(code: &str) -> Result<(), RequestError> {
    let client = create_api_client();

    let req_config = RequestBuilder::new()
        .header("Content-Type", "application/json")
        .build();

    let response_text = client
        .delete(&format!("/api/v1/project/{}", code), Some(req_config))
        .await?;

    let api_response: ApiResponse<String> = serde_json::from_str(&response_text)
        .map_err(|e| RequestError::deserialization_error(e.to_string()))?;

    if !api_response.result {
        return Err(RequestError::api_error(api_response.msg));
    }

    Ok(())
}
