use dioxus::prelude::*;
use crate::models::collection::TaskStatus;

/// T050: TaskStatusBadge component - Display task status with color coding
#[component]
pub fn TaskStatusBadge(status: TaskStatus) -> Element {
    let (badge_class, status_text) = match status {
        TaskStatus::Draft => ("badge-ghost", "Draft"),
        TaskStatus::Saved => ("badge-info", "Saved"),
        TaskStatus::Applied => ("badge-primary", "Applied"),
        TaskStatus::Running => ("badge-accent", "Running"),
        TaskStatus::Failed => ("badge-error", "Failed"),
    };

    rsx! {
        span { class: "badge {badge_class}", "{status_text}" }
    }
}
