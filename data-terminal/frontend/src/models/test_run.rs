use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::Display;

/// Test run status enum
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug, Display)]
#[serde(rename_all = "lowercase")]
pub enum TestRunStatus {
    Pending,
    Running,
    Success,
    Failed,
}

impl Default for TestRunStatus {
    fn default() -> Self {
        TestRunStatus::Pending
    }
}

/// Test run status response
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct TestRunStatusResponse {
    pub status: TestRunStatus,
    pub steps: Vec<TestStepInfo>,
}

/// Test step info
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct TestStepInfo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: TestRunStatus,
    pub error_message: Option<String>,
    pub start_time: Option<String>,  // ISO 8601 format
    pub end_time: Option<String>,    // ISO 8601 format
}

/// Test log level enum
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug, Display)]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl LogLevel {
    pub fn css_class(&self) -> &'static str {
        match self {
            LogLevel::Info => "text-blue-600",
            LogLevel::Success => "text-green-600",
            LogLevel::Warning => "text-yellow-600",
            LogLevel::Error => "text-red-600",
        }
    }
}

/// Test log response
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct TestLogResponse {
    pub total: i64,
    pub items: Vec<TestLogItem>,
}

/// Test log item
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct TestLogItem {
    pub timestamp: String,  // ISO 8601 format
    pub level: LogLevel,
    pub message: String,
    pub details: Option<String>,
}

/// Table preview response
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct TablePreviewResponse {
    pub table_name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

/// Test run task ID response
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct TestRunTaskIdResponse {
    pub task_id: String,
}
