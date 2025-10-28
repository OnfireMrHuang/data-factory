use serde::{Deserialize, Serialize};
use std::time::Duration;

/// HTTP client for communicating with data-engine module
pub struct DataEngineClient {
    base_url: String,
    http_client: reqwest::Client,
}

impl DataEngineClient {
    pub fn new(base_url: String) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url,
            http_client,
        }
    }

    /// Submit a collection task to data-engine for execution
    pub async fn submit_task(
        &self,
        task_config: CollectionTaskConfig,
    ) -> Result<SubmitTaskResponse, DataEngineError> {
        let url = format!("{}/api/v1/pipeline/tasks", self.base_url);

        let response = self
            .http_client
            .post(&url)
            .json(&task_config)
            .send()
            .await
            .map_err(|e| DataEngineError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(DataEngineError::ApiError(format!(
                "Failed to submit task: {}",
                response.status()
            )));
        }

        let result = response
            .json::<SubmitTaskResponse>()
            .await
            .map_err(|e| DataEngineError::ParseError(e.to_string()))?;

        Ok(result)
    }

    /// Get task execution status
    pub async fn get_task_status(
        &self,
        execution_id: &str,
    ) -> Result<TaskStatus, DataEngineError> {
        let url = format!(
            "{}/api/v1/pipeline/tasks/{}/status",
            self.base_url, execution_id
        );

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| DataEngineError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(DataEngineError::ApiError(format!(
                "Failed to get task status: {}",
                response.status()
            )));
        }

        let result = response
            .json::<TaskStatus>()
            .await
            .map_err(|e| DataEngineError::ParseError(e.to_string()))?;

        Ok(result)
    }

    /// Cancel a running task
    pub async fn cancel_task(&self, execution_id: &str) -> Result<(), DataEngineError> {
        let url = format!("{}/api/v1/pipeline/tasks/{}", self.base_url, execution_id);

        let response = self
            .http_client
            .delete(&url)
            .send()
            .await
            .map_err(|e| DataEngineError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(DataEngineError::ApiError(format!(
                "Failed to cancel task: {}",
                response.status()
            )));
        }

        Ok(())
    }
}

// ============================================================================
// Data Engine API Types
// ============================================================================

#[derive(Debug, Serialize)]
pub struct CollectionTaskConfig {
    pub task_type: String,
    pub task_name: String,
    pub task_config: serde_json::Value,
    pub submitted_by: String,
}

#[derive(Debug, Deserialize)]
pub struct SubmitTaskResponse {
    pub execution_id: String,
    pub status: String,
    pub submitted_at: String,
}

#[derive(Debug, Deserialize)]
pub struct TaskStatus {
    pub execution_id: String,
    pub status: String,
    pub progress_percentage: Option<f32>,
    pub current_step: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum DataEngineError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}
