use dioxus::prelude::*;
use crate::models::collection::TableMetadata;

/// T047: DbConfigPanel component - Configure database collection (table/field selection)
#[component]
pub fn DbConfigPanel(
    tables: Vec<TableMetadata>,
    selected_tables: Signal<Vec<String>>,
    on_table_toggle: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "card bg-base-200",
            div { class: "card-body",
                h3 { class: "card-title", "Database Configuration" }
                p { class: "text-sm opacity-70 mb-4",
                    "Select tables and fields to collect from source database"
                }

                // TODO: Implement table selection list with checkboxes
                // TODO: For each selected table, show field selector
                // TODO: Add "Select All" / "Deselect All" buttons

                div { class: "overflow-y-auto max-h-96",
                    if tables.is_empty() {
                        div { class: "alert alert-info",
                            "No tables found. Select a datasource first."
                        }
                    } else {
                        for table in tables {
                            div { class: "form-control",
                                label { class: "label cursor-pointer",
                                    span { class: "label-text", "{table.table_name}" }
                                    input {
                                        r#type: "checkbox",
                                        class: "checkbox checkbox-primary",
                                        checked: selected_tables().contains(&table.table_name),
                                        onchange: move |_| {
                                            on_table_toggle.call(table.table_name.clone());
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
