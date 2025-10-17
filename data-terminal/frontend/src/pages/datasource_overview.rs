use dioxus::prelude::*;
use crate::routes::Route;
use crate::components::datasource_card::DatasourceCard;
use crate::components::datasource_type_dialog::DataSourceTypeDialog;
use crate::pages::datasource_mysql_config::{MysqlConfig};
use crate::models::{datasource::*, protocol::*};
use crate::utils::error;
use crate::utils::request::HttpRequest;
use crate::utils::{cookie, request::RequestBuilder};
use dioxus_free_icons::{icons::hi_outline_icons::*, Icon};
use dioxus_toast::{Icon, ToastInfo, ToastManager};
use tracing::info;

#[component]
pub fn DatasourceOverViewPage() -> Element {

    let navigator = use_navigator();

    let datasources = use_signal(|| Vec::<DataSource>::new());
    let mut error_msg = use_signal(|| String::new());
    let mut selected_ds = use_signal(|| None as Option<DataSource>);
    let mut show_test = use_signal(|| false);
    let mut show_edit = use_signal(|| false);
    let mut show_delete = use_signal(|| false);

    // New dialog states
    let mut show_type_dialog = use_signal(|| false);
    let mut selected_type = use_signal(|| None as Option<DataSourceType>);

    // 搜索状态
    let mut category_filter = use_signal(String::new);
    let mut type_filter = use_signal(String::new);
    let mut status_filter = use_signal(String::new);
    let mut name_filter = use_signal(String::new);
    
    // 过滤后的数据源
    let filtered_datasources = use_memo(move || {
        let mut result = datasources().clone();

        if !category_filter().is_empty() {
            result.retain(|ds| {
                ds.category.to_string() == category_filter()
            });
        }

        if !type_filter().is_empty() {
            result.retain(|ds| {
                ds.datasource_type.to_string() == type_filter()
            });
        }

        if !status_filter().is_empty() {
            result.retain(|ds| {
                ds.connection_status.to_string() == status_filter()
            });
        }

        if !name_filter().is_empty() {
            result.retain(|ds| {
                ds.name.to_lowercase().contains(&name_filter().to_lowercase())
            });
        }
        result
    });

    // 获取数据源列表
    let fetch_datasources = {
        let mut datasources = datasources.clone();
        move || {
            spawn(async move {
            let client = crate::utils::request::create_client("http://localhost:3000");
            let req_config = RequestBuilder::new()
                .header("Content-Type", "application/json")
                .header("Cookie", &cookie::get_browser_cookies())
                .query_param("page", 1)
                .query_param("page_size", 100)
                .build();
            let response = client.get("/api/v1/datasource/list", Some(req_config)).await;
            match response {
                Ok(response_text) => {
                    info!("Received response: {}", response_text);
                    let result = serde_json::from_str::<ApiResponse<Vec<DataSource>>>(&response_text);
                    match result {
                        Ok(api_response) => {
                            info!("API response success: {}, data count: {}", 
                                api_response.result, api_response.data.len());
                            if api_response.result {
                                datasources.set(api_response.data);
                            } else {
                                error_msg.set(api_response.msg.clone());
                            }
                        }
                        Err(e) => {
                            error_msg.set(e.to_string());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Request failed: {}", e);
                    error_msg.set(e.to_string());
                }
            }
        })
        }
    };

    // 组件挂载时获取数据源列表
    use_effect(move || {
        fetch_datasources();
    });
    
    // 搜索处理函数
    let handle_search = move |_| {
        // 搜索逻辑已在use_signal中处理
        info!("执行搜索操作");
    };
    
    // 测试连接
    let handle_test_connection = move |id: String| {
        if let Some(ds) = datasources().iter().find(|d| d.id == id).cloned() {
            selected_ds.set(Some(ds));
            show_test.set(true);
        }
    };
    
    // 编辑数据源
    let handle_edit = move |id: String| {
        if let Some(ds) = datasources().iter().find(|d| d.id == id).cloned() {
            selected_ds.set(Some(ds));
            show_edit.set(true);
        }
    };
    
    // 删除数据源
    let handle_delete = move |id: String| {
        if let Some(ds) = datasources().iter().find(|d| d.id == id).cloned() {
            selected_ds.set(Some(ds));
            show_delete.set(true);
        }
    };

    // 添加数据源 - Updated to show type dialog
    let handle_add = move |_| {
        show_type_dialog.set(true);
    };

    // Handle datasource type selection
    let handle_type_select = move |ds_type: DataSourceType| {
        selected_type.set(Some(ds_type));
        show_type_dialog.set(false);
        navigator.push(Route::DatasourceMysqlAdd{});
    };

    // Handle MySQL connection test
    let handle_test_mysql = move |config: MysqlConfig| {
        info!("Testing MySQL connection: {}@{}:{}/{}",
            config.username, config.host, config.port, config.database);
        // TODO: Implement actual connection test via backend API
    };


    // Handle type dialog close
    let handle_type_close = move |_| {
        show_type_dialog.set(false);
    };

    rsx! {
            div { class: "p-6 space-y-6",
                // 页面标题
                div { class: "flex justify-between items-center",
                    h1 { class: "text-2xl font text-gray-800", "数据源管理" }
                    button {
                        class: "btn btn-info",
                        onclick: handle_add,
                        Icon { icon: HiPlus, class: "w-4 h-4 mr-2" }
                        "添加数据源"
                    }
                }
                
                // 搜索栏
                div { class: "card bg-base-100 shadow-sm w-full",
                    div { class: "card-body",
                        div { class: "flex items-end gap-4 justify-end w-full",
                            // 数据源分类查找框
                            div { class: "form-control",
                                select {
                                    class: "select select-bordered w-full",
                                    value: "{category_filter}",
                                    onchange: move |e| category_filter.set(e.value().to_string()),
                                    option { value: "", disabled: true, selected: category_filter().is_empty(), "数据源分类" }
                                    option { value: "", "全部分类" }
                                    option { value: "Database", "数据库" }
                                    option { value: "Api", "API" }
                                }
                            }

                            // 数据源类型查找框
                            div { class: "form-control",
                                select {
                                    class: "select select-bordered w-full",
                                    value: "{type_filter}",
                                    onchange: move |e| type_filter.set(e.value().to_string()),
                                    option { value: "", disabled: true, selected: type_filter().is_empty(), "数据源类型" }
                                    option { value: "", "全部类型" }
                                    option { value: "Mysql", "MySQL" }
                                    option { value: "Postgres", "PostgreSQL" }
                                    option { value: "QueryApi", "查询API" }
                                    option { value: "SubscribeApi", "订阅API" }
                                }
                            }

                            // 数据源状态搜索框
                            div { class: "form-control",
                                select {
                                    class: "select select-bordered w-full",
                                    value: "{status_filter}",
                                    onchange: move |e| status_filter.set(e.value().to_string()),
                                    option { value: "", disabled: true, selected: status_filter().is_empty(), "连接状态" }
                                    option { value: "", "全部状态" }
                                    option { value: "Connected", "已连接" }
                                    option { value: "Disconnected", "未连接" }
                                    option { value: "Error", "连接错误" }
                                }
                            }

                            // 数据源名称搜索框
                            div { class: "form-control",
                                input {
                                    class: "input input-bordered w-full",
                                    placeholder: "数据源名称",
                                    value: "{name_filter}",
                                    oninput: move |e| name_filter.set(e.value().to_string())
                                }
                            }

                            // 搜索按钮
                            div { class: "form-control",
                                button {
                                    class: "btn btn-info",
                                    onclick: handle_search,
                                    "搜索"
                                }
                            }
                        }
                    }
                }

                
                // 如果有错误信息
                if !error_msg().is_empty() {
                    div {
                        class: "flex justify-center items-center my-12",
                        span {
                            class: "text-error text-lg font-medium",
                            "{error_msg()}"
                        }
                    }
                } else {
                    // 数据源卡片列表
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                        for datasource in filtered_datasources().iter() {
                            DatasourceCard {
                                datasource: datasource.clone(),
                                on_test_connection: handle_test_connection,
                                on_edit: handle_edit,
                                on_delete: handle_delete,
                            }
                        }
                    }
                }
            }

            // Datasource Type Selection Dialog
            DataSourceTypeDialog {
                show: show_type_dialog,
                on_select: handle_type_select,
                on_close: handle_type_close,
            }
    }
}
