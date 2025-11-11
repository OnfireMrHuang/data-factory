use dioxus::prelude::*;

/// T044: ModeSelector component - Choose between Full and Incremental collection
#[component]
pub fn ModeSelector(
    selected_mode: Signal<Option<String>>,
    on_mode_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "form-control w-full",
            label { class: "label",
                span { class: "label-text font-semibold", "Collection Mode" }
            }
            div { class: "flex gap-4",
                // Full Collection Option
                button {
                    class: "btn btn-outline flex-1",
                    class: if selected_mode() == Some("full".to_string()) { "btn-primary" } else { "" },
                    onclick: move |_| {
                        selected_mode.set(Some("full".to_string()));
                        on_mode_change.call("full".to_string());
                    },
                    div { class: "text-left",
                        div { class: "font-bold", "Full Collection" }
                        div { class: "text-xs opacity-70", "Batch ETL - complete data transfer" }
                    }
                }

                // Incremental Collection Option
                button {
                    class: "btn btn-outline flex-1",
                    class: if selected_mode() == Some("incremental".to_string()) { "btn-primary" } else { "" },
                    onclick: move |_| {
                        selected_mode.set(Some("incremental".to_string()));
                        on_mode_change.call("incremental".to_string());
                    },
                    div { class: "text-left",
                        div { class: "font-bold", "Incremental Collection" }
                        div { class: "text-xs opacity-70", "Real-time CDC/streaming" }
                    }
                }
            }
        }
    }
}
