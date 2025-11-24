use dioxus::prelude::*;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// Test run step status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestStepStatus {
    Pending,
    Running,
    Success,
    Failed,
    Skipped,
}

/// Test run step data
#[derive(Debug, Clone, PartialEq)]
pub struct TestStep {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub status: TestStepStatus,
    pub error_message: Option<String>,
    pub start_time: Option<DateTime<Local>>,
    pub end_time: Option<DateTime<Local>>,
}

/// Log level for test run
#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
}

/// Log entry for test run
#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub message: String,
    pub details: Option<String>,
}

/// Test run timeline component
#[component]
pub fn TestRunTimeline(
    steps: Vec<TestStep>,
    current_step: usize,
) -> Element {
    rsx! {
        div { class: "w-full",
            ul { class: "timeline timeline-vertical",
                for (index, step) in steps.iter().enumerate() {
                    li { key: "{step.id}",
                        if index > 0 {
                            hr {
                                class: match step.status {
                                    TestStepStatus::Success => "bg-success",
                                    TestStepStatus::Failed => "bg-error",
                                    TestStepStatus::Running => "bg-primary",
                                    _ => ""
                                }
                            }
                        }

                        div { class: "timeline-start timeline-box",
                            div { class: "flex items-center gap-2",
                                // Status icon
                                match step.status {
                                    TestStepStatus::Pending => rsx! {
                                        span { class: "loading loading-ring loading-sm text-gray-400" }
                                    },
                                    TestStepStatus::Running => rsx! {
                                        span { class: "loading loading-spinner loading-sm text-primary" }
                                    },
                                    TestStepStatus::Success => rsx! {
                                        svg { class: "w-5 h-5 text-success",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke_width: "2",
                                            stroke: "currentColor",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M5 13l4 4L19 7"
                                            }
                                        }
                                    },
                                    TestStepStatus::Failed => rsx! {
                                        svg { class: "w-5 h-5 text-error",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke_width: "2",
                                            stroke: "currentColor",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M6 18L18 6M6 6l12 12"
                                            }
                                        }
                                    },
                                    TestStepStatus::Skipped => rsx! {
                                        svg { class: "w-5 h-5 text-gray-400",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke_width: "2",
                                            stroke: "currentColor",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                d: "M8 7h8m0 0v8m0-8l-8 8"
                                            }
                                        }
                                    }
                                }

                                div {
                                    div { class: "font-semibold", "{step.title}" }
                                    div { class: "text-sm text-gray-500", "{step.description}" }
                                    if let Some(error_msg) = &step.error_message {
                                        div { class: "text-sm text-error mt-1", "{error_msg}" }
                                    }
                                    if let Some(start_time) = step.start_time {
                                        div { class: "text-xs text-gray-400 mt-1",
                                            {
                                                let time_str = start_time.format("%H:%M:%S").to_string();
                                                if let Some(end_time) = step.end_time {
                                                    format!("{} - {}", time_str, end_time.format("%H:%M:%S"))
                                                } else {
                                                    time_str
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div {
                            class: format!("timeline-middle {}",
                                if index == current_step { "text-primary" } else { "" }
                            ),
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 20 20",
                                fill: "currentColor",
                                class: "w-5 h-5",
                                path {
                                    fill_rule: "evenodd",
                                    d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                                    clip_rule: "evenodd"
                                }
                            }
                        }

                        if index < steps.len() - 1 {
                            hr {
                                class: match steps.get(index + 1).map(|s| &s.status) {
                                    Some(TestStepStatus::Success) => "bg-success",
                                    Some(TestStepStatus::Failed) => "bg-error",
                                    Some(TestStepStatus::Running) => "bg-primary",
                                    _ => ""
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Test run log viewer component
#[component]
pub fn TestRunLogViewer(
    logs: Vec<LogEntry>,
    max_height: Option<String>,
) -> Element {
    let height = max_height.unwrap_or_else(|| "400px".to_string());

    rsx! {
        div { class: "w-full",
            div { class: "card bg-base-200",
                div { class: "card-body p-4",
                    h3 { class: "font-semibold mb-3", "执行日志" }
                    div {
                        class: "overflow-auto font-mono text-sm bg-base-300 rounded p-3",
                        style: "max-height: {height}; min-height: 200px;",
                        if logs.is_empty() {
                            div { class: "text-gray-500", "等待执行..." }
                        } else {
                            for log in logs.iter() {
                                div { class: "mb-2",
                                    div { class: "flex items-start gap-2",
                                        span { class: "text-gray-400 text-xs whitespace-nowrap",
                                            {log.timestamp.format("%H:%M:%S%.3f").to_string()}
                                        }
                                        span {
                                            class: match log.level {
                                                LogLevel::Info => "text-info",
                                                LogLevel::Success => "text-success",
                                                LogLevel::Warning => "text-warning",
                                                LogLevel::Error => "text-error",
                                            },
                                            match log.level {
                                                LogLevel::Info => "[INFO]",
                                                LogLevel::Success => "[SUCCESS]",
                                                LogLevel::Warning => "[WARN]",
                                                LogLevel::Error => "[ERROR]",
                                            }
                                        }
                                        span { class: "flex-1", "{log.message}" }
                                    }
                                    if let Some(details) = &log.details {
                                        div { class: "ml-16 text-gray-500 text-xs mt-1 whitespace-pre-wrap",
                                            "{details}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Data preview table component
#[component]
pub fn DataPreviewTable(
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
    table_name: String,
) -> Element {
    rsx! {
        div { class: "w-full",
            div { class: "card bg-base-200",
                div { class: "card-body",
                    h3 { class: "font-semibold mb-3",
                        "数据预览 - {table_name}"
                        span { class: "text-sm text-gray-500 ml-2",
                            "({rows.len()} 行)"
                        }
                    }
                    div { class: "overflow-x-auto",
                        table { class: "table table-zebra",
                            thead {
                                tr {
                                    for col in columns.iter() {
                                        th { "{col}" }
                                    }
                                }
                            }
                            tbody {
                                if rows.is_empty() {
                                    tr {
                                        td {
                                            colspan: "{columns.len()}",
                                            class: "text-center text-gray-500",
                                            "暂无数据"
                                        }
                                    }
                                } else {
                                    for row in rows.iter() {
                                        tr {
                                            for cell in row.iter() {
                                                td {
                                                    class: "max-w-xs truncate",
                                                    title: "{cell}",
                                                    "{cell}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}