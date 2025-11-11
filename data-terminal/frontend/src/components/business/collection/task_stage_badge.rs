use dioxus::prelude::*;
use crate::models::collection::TaskStage;

/// T050: TaskStatusBadge component - Display task status with color coding
#[component]
pub fn TaskStageBadge(stage: TaskStage) -> Element {
    let (badge_class, status_text) = match stage {
        TaskStage::Draft => ("badge-ghost", "Draft"),
        TaskStage::Applied => ("badge-primary", "Applied"),
    };

    rsx! {
        span { class: "badge {badge_class}", "{status_text}" }
    }
}
