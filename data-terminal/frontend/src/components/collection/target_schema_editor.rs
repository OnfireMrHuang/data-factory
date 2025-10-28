use dioxus::prelude::*;
use crate::models::collection::TableSchema;

/// T049: TargetSchemaEditor component - Display and edit target schema
#[component]
pub fn TargetSchemaEditor(
    schema: Signal<Option<TableSchema>>,
    on_schema_change: EventHandler<TableSchema>,
) -> Element {
    rsx! {
        div { class: "card bg-base-100",
            div { class: "card-body",
                h3 { class: "card-title", "Target Schema" }

                if let Some(s) = schema() {
                    div {
                        div { class: "mb-4",
                            label { class: "label",
                                span { class: "label-text", "Target Table Name" }
                            }
                            input {
                                r#type: "text",
                                class: "input input-bordered w-full",
                                value: "{s.table_name}",
                                readonly: true
                            }
                        }

                        // Fields table
                        div { class: "overflow-x-auto",
                            table { class: "table table-zebra",
                                thead {
                                    tr {
                                        th { "Field Name" }
                                        th { "Type" }
                                        th { "Nullable" }
                                        th { "Primary Key" }
                                    }
                                }
                                tbody {
                                    for field in &s.fields {
                                        tr {
                                            td { "{field.field_name}" }
                                            td { code { "{field.field_type}" } }
                                            td {
                                                if field.nullable {
                                                    span { class: "badge badge-warning", "YES" }
                                                } else {
                                                    span { class: "badge badge-success", "NO" }
                                                }
                                            }
                                            td {
                                                if field.primary_key {
                                                    span { class: "badge badge-primary", "PK" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "alert alert-info",
                        "Select tables to generate target schema"
                    }
                }
            }
        }
    }
}
