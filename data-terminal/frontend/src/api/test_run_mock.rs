use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use gloo::timers::future::sleep;

/// Mock test run request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunRequest {
    pub task_name: String,
    pub datasource_id: String,
    pub database_resource_id: String,
    pub ddl_sql: String,
    pub select_sql: String,
}

/// Mock test run response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunResponse {
    pub success: bool,
    pub message: String,
    pub test_run_id: String,
}

/// Mock test step result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStepResult {
    pub step_id: usize,
    pub success: bool,
    pub message: String,
    pub error_message: Option<String>,
    pub data: Option<serde_json::Value>,
}

/// Mock preview data response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewDataResponse {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub total_rows: usize,
}

/// Mock API for test run
pub struct TestRunMockApi;

impl TestRunMockApi {
    /// Start test run
    pub async fn start_test_run(_request: TestRunRequest) -> Result<TestRunResponse, String> {
        // Simulate API delay
        sleep(Duration::from_millis(500)).await;

        Ok(TestRunResponse {
            success: true,
            message: "Test run started successfully".to_string(),
            test_run_id: "test_run_12345".to_string(),
        })
    }

    /// Check database connection
    pub async fn check_database_connection(datasource_id: &str) -> Result<TestStepResult, String> {
        // Simulate API delay
        sleep(Duration::from_millis(1000)).await;

        // Always succeed for demo (deterministic behavior)
        let success = true;

        Ok(TestStepResult {
            step_id: 1,
            success,
            message: if success {
                format!("Successfully connected to database: {}", datasource_id)
            } else {
                format!("Failed to connect to database: {}", datasource_id)
            },
            error_message: if !success {
                Some("Connection timeout: Unable to establish connection to MySQL server".to_string())
            } else {
                None
            },
            data: if success {
                Some(json!({
                    "server_version": "8.0.32",
                    "connection_time_ms": 234,
                    "charset": "utf8mb4"
                }))
            } else {
                None
            },
        })
    }

    /// Create temporary table
    pub async fn create_temp_table(ddl_sql: &str) -> Result<TestStepResult, String> {
        // Simulate API delay
        sleep(Duration::from_millis(800)).await;

        let temp_table_name = format!("temp_collection_test_{}", chrono::Utc::now().timestamp());

        Ok(TestStepResult {
            step_id: 2,
            success: true,
            message: format!("Temporary table created: {}", temp_table_name),
            error_message: None,
            data: Some(json!({
                "table_name": temp_table_name,
                "columns_created": 5,
                "ddl_executed": ddl_sql
            })),
        })
    }

    /// Check field consistency
    pub async fn check_field_consistency(select_sql: &str, ddl_sql: &str) -> Result<TestStepResult, String> {
        // Simulate API delay
        sleep(Duration::from_millis(600)).await;

        Ok(TestStepResult {
            step_id: 3,
            success: true,
            message: "Field consistency check passed".to_string(),
            error_message: None,
            data: Some(json!({
                "source_fields": ["id", "name", "created_at", "status", "amount"],
                "target_fields": ["id", "name", "created_at", "status", "amount"],
                "matched_fields": 5,
                "unmatched_fields": 0
            })),
        })
    }

    /// Execute single collection
    pub async fn execute_single_collection(select_sql: &str) -> Result<TestStepResult, String> {
        // Simulate API delay
        sleep(Duration::from_millis(1200)).await;

        Ok(TestStepResult {
            step_id: 4,
            success: true,
            message: "Successfully collected 20 rows of data".to_string(),
            error_message: None,
            data: Some(json!({
                "rows_collected": 20,
                "execution_time_ms": 456,
                "query": select_sql
            })),
        })
    }

    /// Preview temporary table data
    pub async fn preview_temp_table(table_name: &str) -> Result<PreviewDataResponse, String> {
        // Simulate API delay
        sleep(Duration::from_millis(500)).await;

        // Generate mock data
        let columns = vec![
            "id".to_string(),
            "name".to_string(),
            "created_at".to_string(),
            "status".to_string(),
            "amount".to_string(),
        ];

        let mut rows = Vec::new();
        for i in 1..=20 {
            rows.push(vec![
                i.to_string(),
                format!("Product {}", i),
                format!("2024-01-{:02} 10:30:00", i),
                if i % 2 == 0 { "active" } else { "inactive" }.to_string(),
                format!("{:.2}", i as f64 * 100.50),
            ]);
        }

        Ok(PreviewDataResponse {
            columns,
            rows,
            total_rows: 20,
        })
    }

    /// Delete temporary table
    pub async fn delete_temp_table(table_name: &str) -> Result<TestStepResult, String> {
        // Simulate API delay
        sleep(Duration::from_millis(400)).await;

        Ok(TestStepResult {
            step_id: 6,
            success: true,
            message: format!("Temporary table {} has been deleted", table_name),
            error_message: None,
            data: Some(json!({
                "table_deleted": table_name,
                "cleanup_time_ms": 123
            })),
        })
    }
}