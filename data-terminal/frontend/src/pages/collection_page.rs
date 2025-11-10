use dioxus::prelude::*;
use crate::routes::Route;
use crate::models::collection::*;
use crate::utils::collection_api;
// FIXME: dioxus-free-icons doesn't support Dioxus 0.7 yet
// use dioxus_free_icons::{icons::hi_outline_icons::*, Icon};

/// T052: CollectionPage - List all collection tasks
#[component]
pub fn CollectionPage() -> Element {
    let navigator = use_navigator();

    let mut tasks = use_signal(|| Vec::<CollectTask>::new());
    let mut loading = use_signal(|| true);
    let mut error_msg = use_signal(|| String::new());

    // Pagination
    let mut current_page = use_signal(|| 1u32);
    let mut page_size = use_signal(|| 20u32);
    let mut total_items = use_signal(|| 0u32);

    // Filters
    let mut status_filter = use_signal(String::new);
    let mut category_filter = use_signal(String::new);

    // Page input for direct navigation
    let mut page_input = use_signal(String::new);

    // Load tasks on mount and when filters/page change
    use_effect(move || {
        spawn(async move {
            loading.set(true);
            match collection_api::fetch_collection_tasks(
                Some(current_page()),
                Some(page_size()),
                None,
                None,
                None
            ).await {
                Ok(response) => {
                    tasks.set(response.data);
                    total_items.set(response.pagination.total);
                    error_msg.set(String::new());
                }
                Err(e) => {
                    error_msg.set(format!("Failed to load tasks: {:?}", e));
                }
            }
            loading.set(false);
        });
    });

    // Filtered tasks
    let filtered_tasks = use_memo(move || {
        let mut result = tasks().clone();

        if !status_filter().is_empty() {
            result.retain(|t| format!("{:?}", t.stage).to_lowercase() == status_filter().to_lowercase());
        }

        if !category_filter().is_empty() {
            result.retain(|t| format!("{:?}", t.category).to_lowercase() == category_filter().to_lowercase());
        }

        result
    });

    // Calculate total pages
    let total_pages = use_memo(move || {
        let total = total_items();
        let size = page_size();
        if total == 0 {
            1
        } else {
            (total + size - 1) / size
        }
    });

    rsx! {
        div { class: "h-screen flex flex-col overflow-hidden",
            div { class: "container mx-auto p-6 flex-1 flex flex-col overflow-hidden",
                // Header
                div { class: "flex justify-between items-center mb-6",
                    h1 { class: "text-3xl font-bold", "Collection Tasks" }
                    button {
                        class: "btn btn-primary gap-2",
                        onclick: move |_| {
                            navigator.push(Route::CollectionCreatePage {});
                        },
                        span { "➕" }
                        "创建采集任务"
                    }
                }

            // Filters
            div { class: "card bg-base-200 mb-4",
                div { class: "card-body",
                    div { class: "flex gap-4",
                        // Status filter
                        select {
                            class: "select select-bordered",
                            value: "{status_filter}",
                            onchange: move |evt| status_filter.set(evt.value()),
                            option { value: "", "全部状态" }
                            option { value: "draft", "开发态" }
                            option { value: "applied", "应用态" }
                        }

                        // Category filter
                        select {
                            class: "select select-bordered",
                            value: "{category_filter}",
                            onchange: move |evt| category_filter.set(evt.value()),
                            option { value: "", "所有分类" }
                            option { value: "database", "数据库采集" }
                            option { value: "api", "API采集" }
                            option { value: "crawler", "爬虫采集" }
                        }
                    }
                }
            }

            // Error message
            if !error_msg().is_empty() {
                div { class: "alert alert-error mb-4",
                    "{error_msg()}"
                }
            }

            // Loading state
            if loading() {
                div { class: "flex justify-center py-12 flex-1",
                    span { class: "loading loading-spinner loading-lg" }
                }
            } else if filtered_tasks().is_empty() {
                div { class: "alert alert-info flex-1",
                    "No collection tasks found. Click 'Create Collection Task' to get started."
                }
            } else {
                // Tasks table with flex-1 to push pagination to bottom
                div { class: "flex-1 flex flex-col min-h-0",
                    div { class: "flex-1 overflow-auto",
                    table { class: "table table-zebra",
                        thead {
                            tr {
                                th { "名称" }
                                th { "分类" }
                                th { "类型" }
                                th { "状态" }
                                th { "数据源" }
                                th { "资源" }
                                th { "创建时间" }
                                th { "更多" }
                            }
                        }
                        tbody {
                            for task in filtered_tasks() {
                                tr {
                                    key: "{task.id}",
                                    td {
                                        div { class: "font-semibold", "{task.name}" }
                                    }
                                    td {
                                        match task.category {
                                            crate::models::collection::CollectionCategory::Database => rsx! {
                                                span { class: "badge badge-outline badge-primary",
                                                    "数据库采集"
                                                }
                                            },
                                            crate::models::collection::CollectionCategory::Api => rsx! {
                                                span { class: "badge badge-outline badge-success",
                                                    "API采集"
                                                }
                                            },
                                            crate::models::collection::CollectionCategory::Crawler => rsx! {
                                                span { class: "badge badge-outline badge-warning",
                                                    "爬虫采集"
                                                }
                                            },
                                        }
                                    }
                                    td { 
                                        // 类型名称映射
                                        {
                                            let type_name = match task.collect_type {
                                                crate::models::collection::CollectType::Full => "全量采集",
                                                crate::models::collection::CollectType::Incremental => "增量采集",
                                            };
                                            rsx! { "{type_name}" }
                                        }
                                    }
                                    // TaskStage 显示名映射
                                    td {
                                        {
                                            let stage_display = match task.stage {
                                                crate::models::collection::TaskStage::Draft => "开发态",
                                                crate::models::collection::TaskStage::Applied => "应用态",
                                            };
                                            rsx! {
                                                span { class: "badge badge-info", "{stage_display}" }
                                            }
                                        }
                                    }
                                    td {
                                        div { class: "text-sm", "{task.datasource_name}" }
                                    }
                                    td {
                                        div { class: "text-sm", "{task.resource_name}" }
                                    }
                                    td {
                                        "{task.created_at.format(\"%Y-%m-%d\")}"
                                    }
                                    td {
                                        div { class: "flex gap-2",
                                            {
                                                let edit_id = task.id.clone();
                                                rsx! {
                                                    button {
                                                        class: "btn btn-sm btn-ghost",
                                                        onclick: move |_| {
                                                            navigator.push(Route::CollectionEditPage { id: edit_id.clone() });
                                                        },
                                                        span { "..."}
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

                    // Pagination Footer - flex-shrink-0 to prevent shrinking
                    div { class: "flex-shrink-0 bg-base-100 border-t border-base-300 shadow-lg",
                        div { class: "py-3",
                        div { class: "flex justify-between items-center gap-4",
                            // Page size selector (left)
                            div { class: "flex items-center gap-2 flex-shrink-0",
                                select {
                                    class: "select select-bordered select-sm",
                                    value: "{page_size}",
                                    onchange: move |evt| {
                                        if let Ok(size) = evt.value().parse::<u32>() {
                                            page_size.set(size);
                                            current_page.set(1); // Reset to first page when page size changes
                                        }
                                    },
                                    option { value: "10", "10条/页" }
                                    option { value: "20", "20条/页" }
                                    option { value: "50", "50条/页" }
                                    option { value: "100", "100条/页" }
                                }
                            }

                            // Pagination buttons (center)
                            div { class: "join flex-shrink",
                                button {
                                    class: "join-item btn btn-sm",
                                    disabled: current_page() <= 1,
                                    onclick: move |_| {
                                        if current_page() > 1 {
                                            current_page.set(current_page() - 1);
                                        }
                                    },
                                    "«"
                                }

                                // Page numbers
                                {
                                    let current = current_page();
                                    let total = total_pages();
                                    let mut pages_to_show = Vec::new();

                                    // Always show first page
                                    pages_to_show.push(1);

                                    // Show pages around current
                                    for i in (current.saturating_sub(2))..=(current + 2).min(total) {
                                        if i > 1 && i < total {
                                            pages_to_show.push(i);
                                        }
                                    }

                                    // Always show last page
                                    if total > 1 {
                                        pages_to_show.push(total);
                                    }

                                    pages_to_show.sort();
                                    pages_to_show.dedup();

                                    let mut elements = Vec::new();
                                    let mut prev_page = 0u32;

                                    for page in pages_to_show {
                                        // Add ellipsis if there's a gap
                                        if prev_page > 0 && page > prev_page + 1 {
                                            elements.push(rsx! {
                                                button {
                                                    key: "ellipsis-{prev_page}",
                                                    class: "join-item btn btn-sm btn-disabled",
                                                    "..."
                                                }
                                            });
                                        }

                                        let is_current = page == current;
                                        let page_num = page;

                                        elements.push(rsx! {
                                            button {
                                                key: "page-{page}",
                                                class: if is_current { "join-item btn btn-sm btn-active" } else { "join-item btn btn-sm" },
                                                onclick: move |_| {
                                                    current_page.set(page_num);
                                                },
                                                "{page}"
                                            }
                                        });

                                        prev_page = page;
                                    }

                                    elements.into_iter()
                                }

                                button {
                                    class: "join-item btn btn-sm",
                                    disabled: current_page() >= total_pages(),
                                    onclick: move |_| {
                                        if current_page() < total_pages() {
                                            current_page.set(current_page() + 1);
                                        }
                                    },
                                    "»"
                                }
                            }

                            // Direct page input (rightmost)
                            div { class: "flex items-center gap-2 flex-shrink-0",
                                span { class: "text-sm whitespace-nowrap", "前往" }
                                input {
                                    r#type: "text",
                                    class: "input input-bordered input-sm w-16",
                                    value: "{page_input}",
                                    placeholder: "",
                                    oninput: move |evt| page_input.set(evt.value()),
                                    onkeydown: move |evt| {
                                        if evt.key() == Key::Enter {
                                            if let Ok(page) = page_input().parse::<u32>() {
                                                if page >= 1 && page <= total_pages() {
                                                    current_page.set(page);
                                                    page_input.set(String::new());
                                                }
                                            }
                                        }
                                    }
                                }
                                span { class: "text-sm whitespace-nowrap", "页" }
                            }
                        }

                        // Total items info
                        div { class: "text-sm opacity-70 text-center mt-2",
                            "共{total_items()}条"
                        }
                    }
                    }
                }
            }
            }
        }
    }
}
