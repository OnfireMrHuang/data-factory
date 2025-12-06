use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::impl_sqlx_for_string_enum;

/// Test run status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, strum::Display, strum::EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TestRunStatus {
    Pending,
    Running,
    Success,
    Failed,
}

impl Default for TestRunStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl_sqlx_for_string_enum!(TestRunStatus);

/// Test run model (database record)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TestRun {
    pub id: String,
    pub collection_code: String,
    pub status: TestRunStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Test run step model (database record)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TestRunStep {
    pub id: String,
    pub test_run_id: String,
    pub step_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: TestRunStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Test log level enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl_sqlx_for_string_enum!(LogLevel);

/// Test run log model (database record)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TestRunLog {
    pub id: i64,
    pub created_at: i64,  // Unix timestamp in milliseconds
    pub test_run_id: String,
    pub log_level: LogLevel,
    pub message: String,
    pub details: Option<String>,
}

// ============================================================================
// DTOs (Data Transfer Objects) for API responses
// ============================================================================

/// Test run status response (matches API schema)
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TestRunStatusResponse {
    #[serde(default)]
    pub status: TestRunStatus,
    #[serde(default)]
    pub steps: Vec<TestStepInfo>,
}

/// Test step info for status response
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TestStepInfo {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub status: TestRunStatus,
    pub error_message: Option<String>,
    pub start_time: Option<String>,  // ISO 8601 format
    pub end_time: Option<String>,    // ISO 8601 format
}

impl From<TestRunStep> for TestStepInfo {
    fn from(step: TestRunStep) -> Self {
        Self {
            id: step.step_id,
            title: step.title,
            description: step.description.unwrap_or_default(),
            status: step.status,
            error_message: step.error_message,
            start_time: step.started_at.map(|dt| dt.to_rfc3339()),
            end_time: step.completed_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

/// Test log response
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TestLogResponse {
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub items: Vec<TestLogItem>,
}

/// Test log item
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TestLogItem {
    #[serde(default)]
    pub timestamp: String,  // ISO 8601 format
    #[serde(default)]
    pub level: LogLevel,
    #[serde(default)]
    pub message: String,
    pub details: Option<String>,
}

impl From<TestRunLog> for TestLogItem {
    fn from(log: TestRunLog) -> Self {
        let dt = DateTime::from_timestamp_millis(log.created_at).unwrap_or_default();
        Self {
            timestamp: dt.to_rfc3339(),
            level: log.log_level,
            message: log.message,
            details: log.details,
        }
    }
}

/// Table preview response
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TablePreviewResponse {
    #[serde(default)]
    pub table_name: String,
    #[serde(default)]
    pub columns: Vec<String>,
    #[serde(default)]
    pub rows: Vec<Vec<String>>,
}

/// Test run task ID response
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TestRunTaskIdResponse {
    #[serde(default)]
    pub task_id: String,
}
