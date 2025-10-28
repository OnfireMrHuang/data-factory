use dioxus::prelude::*;

/// T048: TransformEditor component - SQL transformation editor
#[component]
pub fn TransformEditor(
    transform_sql: Signal<String>,
    on_sql_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "form-control w-full",
            label { class: "label",
                span { class: "label-text font-semibold", "SQL Transformation (Optional)" }
            }
            textarea {
                class: "textarea textarea-bordered h-32 font-mono",
                placeholder: "SELECT * FROM source_table WHERE created_at > '2024-01-01'",
                value: "{transform_sql}",
                oninput: move |evt| {
                    let value = evt.value();
                    transform_sql.set(value.clone());
                    on_sql_change.call(value);
                }
            }
            label { class: "label",
                span { class: "label-text-alt",
                    "Use SQL to filter or transform data before collection"
                }
            }
        }
    }
}
