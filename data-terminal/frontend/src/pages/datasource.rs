use dioxus::prelude::*;
use crate::components::framework::Framework;
use crate::components::datasource_card::DatasourceCard;
use crate::components::{
    datasource_add_dialog::AddDatasourceDialog,
    datasource_test_dialog::TestDatasourceDialog,
    datasource_edit_dialog::EditDatasourceDialog,
    datasource_delete_dialog::DeleteDatasourceDialog,
};
use crate::models::datasource::*;
use dioxus_free_icons::{icons::hi_outline_icons::*, Icon};

// Mock测试数据
fn get_mock_datasources() -> Vec<DataSource> {
    vec![
        DataSource {
            id: "1".to_string(),
            name: "MySQL主数据库".to_string(),
            description: "生产环境主数据库，存储用户数据和业务数据".to_string(),
            category: DataSourceCategory::Database,
            datasource_type: DataSourceType::Mysql,
            connection_config: serde_json::json!({
                "host": "192.168.1.100",
                "port": 3306,
                "username": "admin",
                "database": "production_db"
            }),
            connection_status: ConnectionStatus::Connected,
            created_at: "2024-01-15T10:30:00Z".to_string(),
            updated_at: "2024-01-20T14:20:00Z".to_string(),
        },
        DataSource {
            id: "2".to_string(),
            name: "PostgreSQL分析库".to_string(),
            description: "数据分析专用数据库，用于报表和BI分析".to_string(),
            category: DataSourceCategory::Database,
            datasource_type: DataSourceType::Postgres,
            connection_config: serde_json::json!({
                "host": "192.168.1.101",
                "port": 5432,
                "username": "analyst",
                "database": "analytics_db"
            }),
            connection_status: ConnectionStatus::Connected,
            created_at: "2024-01-10T09:15:00Z".to_string(),
            updated_at: "2024-01-18T16:45:00Z".to_string(),
        },
        DataSource {
            id: "3".to_string(),
            name: "用户API接口".to_string(),
            description: "第三方用户管理系统API，提供用户信息查询服务".to_string(),
            category: DataSourceCategory::Api,
            datasource_type: DataSourceType::QueryApi,
            connection_config: serde_json::json!({
                "url": "https://api.example.com/users",
                "method": "GET",
                "timeout": 5000
            }),
            connection_status: ConnectionStatus::Connected,
            created_at: "2024-01-12T11:20:00Z".to_string(),
            updated_at: "2024-01-19T10:30:00Z".to_string(),
        },
        DataSource {
            id: "4".to_string(),
            name: "订单订阅API".to_string(),
            description: "实时订单数据订阅服务，支持WebSocket连接".to_string(),
            category: DataSourceCategory::Api,
            datasource_type: DataSourceType::SubscribeApi,
            connection_config: serde_json::json!({
                "url": "wss://api.example.com/orders/stream",
                "method": "GET",
                "polling_interval": 1000
            }),
            connection_status: ConnectionStatus::Error,
            created_at: "2024-01-14T15:45:00Z".to_string(),
            updated_at: "2024-01-21T08:15:00Z".to_string(),
        },
        DataSource {
            id: "5".to_string(),
            name: "测试MySQL实例".to_string(),
            description: "开发环境测试数据库，用于功能验证".to_string(),
            category: DataSourceCategory::Database,
            datasource_type: DataSourceType::Mysql,
            connection_config: serde_json::json!({
                "host": "192.168.1.102",
                "port": 3306,
                "username": "test_user",
                "database": "test_db"
            }),
            connection_status: ConnectionStatus::Disconnected,
            created_at: "2024-01-16T13:30:00Z".to_string(),
            updated_at: "2024-01-17T17:20:00Z".to_string(),
        },
    ]
}

#[component]
pub fn DatasourcePage() -> Element {
    let mut datasources = use_signal(|| get_mock_datasources());
    let mut selected_ds = use_signal(|| None as Option<DataSource>);
    let mut show_add = use_signal(|| false);
    let mut show_test = use_signal(|| false);
    let mut show_edit = use_signal(|| false);
    let mut show_delete = use_signal(|| false);
    
    // 搜索状态
    let mut category_filter = use_signal(String::new);
    let mut type_filter = use_signal(String::new);
    let mut status_filter = use_signal(String::new);
    let mut name_filter = use_signal(String::new);
    
    // 过滤后的数据源
    let filtered_datasources = use_signal(|| {
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
    
    // 搜索处理函数
    let handle_search = move |_| {
        // 搜索逻辑已在use_signal中处理
        log::info!("执行搜索操作");
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
    
    // 添加数据源
    let handle_add = move |_| {
        show_add.set(true);
    };

    rsx! {
        Framework {
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
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body",
                        div { class: "flex flex-wrap items-end gap-4",
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
                                    class: "btn btn-info w-full",
                                    onclick: handle_search,
                                    "搜索"
                                }
                            }
                        }
                    }
                }
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
                
                // 空状态
                if filtered_datasources().is_empty() {
                    div { class: "text-center py-12",
                        Icon { icon: HiDatabase, class: "w-16 h-16 text-gray-300 mx-auto mb-4" }
                        p { class: "text-gray-500", "暂无数据源" }
                        button {
                            class: "btn btn-primary mt-4",
                            onclick: handle_add,
                            "添加第一个数据源"
                        }
                    }
                }
            }
        }
        // 对话框：添加
        AddDatasourceDialog {
            open: show_add(),
            on_close: move |_| show_add.set(false),
            on_submit: move |form: DataSourceFormData| {
                // 简单追加到本地列表
                let mut list = datasources();
                let new = DataSource{
                    id: format!("{}", list.len() + 1),
                    name: form.name,
                    description: form.description,
                    category: form.category,
                    datasource_type: form.datasource_type,
                    connection_config: form.connection_config,
                    connection_status: ConnectionStatus::Disconnected,
                    created_at: "2024-01-22T00:00:00Z".to_string(),
                    updated_at: "2024-01-22T00:00:00Z".to_string(),
                };
                list.push(new);
                datasources.set(list);
                show_add.set(false);
            }
        }
        // 对话框：测试
        TestDatasourceDialog {
            open: show_test(),
            datasource: selected_ds(),
            on_close: move |_| { show_test.set(false); selected_ds.set(None); },
            on_test: move |id: String| {
                log::info!("开始测试连接: {}", id);
            }
        }
        // 对话框：编辑
        EditDatasourceDialog {
            open: show_edit(),
            datasource: selected_ds(),
            on_close: move |_| { show_edit.set(false); selected_ds.set(None); },
            on_submit: move |upd: DataSourceCreateUpdate| {
                let mut list = datasources();
                if let Some(item) = list.iter_mut().find(|d| d.id == upd.id) {
                    item.name = upd.name;
                    item.description = upd.description;
                    item.category = upd.category;
                    item.datasource_type = upd.datasource_type;
                    item.connection_config = upd.connection_config;
                    item.updated_at = "2024-01-22T00:00:00Z".to_string();
                }
                datasources.set(list);
                show_edit.set(false);
                selected_ds.set(None);
            }
        }
        // 对话框：删除
        DeleteDatasourceDialog {
            open: show_delete(),
            datasource: selected_ds(),
            on_close: move |_| { show_delete.set(false); selected_ds.set(None); },
            on_confirm: move |id: String| {
                let mut list = datasources();
                list.retain(|d| d.id != id);
                datasources.set(list);
                show_delete.set(false);
                selected_ds.set(None);
            }
        }
    }
}