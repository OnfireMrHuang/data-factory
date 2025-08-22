use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::models::resource::{Resource, Category, ResourceType, Status};
use crate::components::{
    resource_dialog::{ResourceDialog, ResourceModalMode},
    resource_delete_dialog::ResourceDeleteDialog,
};
use crate::routes::Route;

#[component]
pub fn ResourcePage() -> Element {
    let navigator = use_navigator();
    let mut selected_category = use_signal(|| Category::RelationalDatabase);
    let mut selected_resource_type = use_signal(|| ResourceType::Mysql);
    
    // 弹窗状态管理
    let mut show_add_dialog = use_signal(|| false);
    let mut show_resource_dialog = use_signal(|| false);
    let mut show_delete_dialog = use_signal(|| false);
    let mut resource_modal_mode = use_signal(|| ResourceModalMode::Add);
    let mut selected_resource_for_action = use_signal(|| None as Option<Resource>);
    
    // 操作菜单状态管理
    let mut show_action_menu = use_signal(|| None as Option<String>);
    let mut menu_position = use_signal(|| "bottom" as &str);

    // 获取资源类型
    let get_resource_types = |category: Category| {
        match category {
            Category::RelationalDatabase => vec![ResourceType::Mysql, ResourceType::Postgres, ResourceType::Doris],
            Category::VectorDatabase => vec![ResourceType::Mailvus],
            Category::Filesystem => vec![ResourceType::Hdfs],
            Category::Queue => vec![ResourceType::Kafka],
            Category::BatchCompute => vec![ResourceType::Spark],
            Category::StreamCompute => vec![ResourceType::Flink],
            _ => vec![],
        }
    };

    // 获取资源图标
    let get_resource_icon = |resource_type: &ResourceType| {
        match resource_type {
            ResourceType::Mysql => include_str!("../../assets/resource/mysql.svg"),
            ResourceType::Postgres => include_str!("../../assets/resource/postgres.svg"),
            ResourceType::Doris => include_str!("../../assets/resource/doris.svg"),
            ResourceType::Mailvus => include_str!("../../assets/resource/milvus.svg"),
            ResourceType::Spark => include_str!("../../assets/resource/spark.svg"),
            ResourceType::Flink => include_str!("../../assets/resource/flink.svg"),
            ResourceType::Kafka => include_str!("../../assets/resource/kafka.svg"),
            ResourceType::Hdfs => include_str!("../../assets/resource/hdfs.svg"),
        }
    };

    // 获取状态颜色
    let get_status_color = |status: &Status| {
        match status {
            Status::Active => "text-green-600 bg-green-100",
            Status::Inactive => "text-red-600 bg-red-100",
        }
    };

    // 获取分类显示名称
    let get_category_name = |category: &Category| {
        match category {
            Category::RelationalDatabase => "关系型数据库",
            Category::TimeSeriesDatabase => "时序数据库",
            Category::DocumentDatabase => "文档数据库",
            Category::VectorDatabase => "向量数据库",
            Category::GraphDatabase => "图数据库",
            Category::KVDatabase => "键值数据库",
            Category::Filesystem => "文件系统",
            Category::Queue => "消息队列",
            Category::BatchCompute => "批处理计算",
            Category::StreamCompute => "流处理计算",
        }
    };

    // 获取资源类型显示名称
    let get_resource_type_name = |resource_type: &ResourceType| {
        match resource_type {
            ResourceType::Mysql => "MySQL",
            ResourceType::Postgres => "PostgreSQL",
            ResourceType::Doris => "Doris",
            ResourceType::Mailvus => "Milvus",
            ResourceType::Spark => "Spark",
            ResourceType::Flink => "Flink",
            ResourceType::Kafka => "Kafka",
            ResourceType::Hdfs => "HDFS",
        }
    };

    // 处理点击外部关闭菜单
    let handle_click_outside = {
        let mut show_action_menu = show_action_menu.clone();
        move |_| {
            show_action_menu.set(None);
        }
    };

    // 处理新增资源
    let handle_add_resource = {
        let mut show_add_dialog = show_add_dialog.clone();
        move |_| {
            show_add_dialog.set(true);
        }
    };

    // 处理资源详情
    let mut handle_resource_detail = {
        let mut show_resource_dialog = show_resource_dialog.clone();
        let mut resource_modal_mode = resource_modal_mode.clone();
        let mut show_action_menu = show_action_menu.clone();
        move |resource: Resource| {
            resource_modal_mode.set(ResourceModalMode::Detail(resource));
            show_resource_dialog.set(true);
            show_action_menu.set(None);
        }
    };

    // 处理资源编辑
    let mut handle_resource_edit = {
        let mut show_resource_dialog = show_resource_dialog.clone();
        let mut resource_modal_mode = resource_modal_mode.clone();
        let mut show_action_menu = show_action_menu.clone();
        move |resource: Resource| {
            resource_modal_mode.set(ResourceModalMode::Edit(resource));
            show_resource_dialog.set(true);
            show_action_menu.set(None);
        }
    };

    // 处理资源删除
    let mut handle_resource_delete = {
        let mut show_delete_dialog = show_delete_dialog.clone();
        let mut selected_resource_for_action = selected_resource_for_action.clone();
        let mut show_action_menu = show_action_menu.clone();
        move |resource: Resource| {
            selected_resource_for_action.set(Some(resource));
            show_delete_dialog.set(true);
            show_action_menu.set(None);
        }
    };

    // 处理删除确认
    let mut handle_delete_confirm = {
        let mut show_delete_dialog = show_delete_dialog.clone();
        move |resource: Resource| {
            // TODO: 调用删除API
            println!("删除资源: {}", resource.name);
            show_delete_dialog.set(false);
        }
    };

    // 处理删除取消
    let mut handle_delete_cancel = {
        let mut show_delete_dialog = show_delete_dialog.clone();
        move |_| {
            show_delete_dialog.set(false);
        }
    };

    rsx! {
        div { 
            class: "flex flex-col h-screen bg-gray-50",
            onclick: handle_click_outside,
            // 导航栏
            div { class: "bg-white shadow-sm border-b border-gray-200 px-6 py-4 flex justify-between items-center",
                div { class: "flex items-center space-x-4",
                    button {
                        class: "text-gray-600 hover:text-gray-900 flex items-center space-x-2",
                        onclick: move |_| {
                            navigator.push(Route::Home {});
                        },
                        span { "←" }
                        span { "返回Home" }
                    }
                }
                div { class: "flex items-center space-x-4",
                    button {
                        class: "bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 flex items-center space-x-2",
                        onclick: handle_add_resource,
                        span { "➕" }
                        span { "新增资源" }
                    }
                }
            }

            // 主体内容
            div { class: "flex flex-1 overflow-hidden",
                // 左侧栏 - 资源分类
                div { class: "w-64 bg-white shadow-sm border-r border-gray-200 overflow-y-auto",
                    div { class: "p-4",
                        h3 { class: "text-lg font-semibold text-gray-900 mb-4", "资源分类" }
                        div { class: "space-y-2",
                            {get_all_categories().into_iter().map(|category| {
                                let is_selected = selected_category() == category;
                                let class_name = if is_selected { 
                                    "w-full text-left px-3 py-2 rounded-lg transition-colors bg-blue-100 text-blue-700" 
                                } else { 
                                    "w-full text-left px-3 py-2 rounded-lg transition-colors text-gray-700 hover:bg-gray-100" 
                                };
                                rsx! {
                                    button {
                                        key: "{category:?}",
                                        class: class_name,
                                        onclick: move |_| {
                                            selected_category.set(category.clone());
                                            if let Some(first_type) = get_resource_types(category.clone()).first() {
                                                selected_resource_type.set(first_type.clone());
                                            }
                                        },
                                        "{get_category_name(&category)}"
                                    }
                                }
                            })}
                        }
                    }
                }

                // 中间内容区域
                div { class: "flex-1 overflow-hidden",
                    div { class: "h-full flex flex-col",
                        // 资源类型选择
                        div { class: "bg-white shadow-sm border-b border-gray-200 px-6 py-4",
                            div { class: "flex items-center space-x-4",
                                h2 { class: "text-xl font-semibold text-gray-900", "资源类型" }
                                div { class: "flex space-x-2",
                                    {get_resource_types(selected_category()).into_iter().map(|resource_type| {
                                        let is_selected = selected_resource_type() == resource_type;
                                        let class_name = if is_selected { 
                                            "px-3 py-1 rounded-md text-sm transition-colors bg-blue-600 text-white" 
                                        } else { 
                                            "px-3 py-1 rounded-md text-sm transition-colors bg-gray-100 text-gray-700 hover:bg-gray-200" 
                                        };
                                        rsx! {
                                            button {
                                                key: "{resource_type:?}",
                                                class: class_name,
                                                onclick: move |_| {
                                                    selected_resource_type.set(resource_type.clone());
                                                },
                                                "{get_resource_type_name(&resource_type)}"
                                            }
                                        }
                                    })}
                                }
                            }
                        }

                        // 资源实例列表
                        div { class: "flex-1 overflow-y-auto p-6",
                            div { class: "space-y-4",
                                {get_mock_resources().into_iter().filter(|resource| {
                                    resource.category == selected_category() && 
                                    resource.resource_type == selected_resource_type()
                                }).enumerate().map(|(index, resource)| {
                                    let status_class = get_status_color(&resource.status);
                                    let icon = get_resource_icon(&resource.resource_type);
                                    let name = resource.name.clone();
                                    let description = resource.description.clone();
                                    let status = format!("{:?}", resource.status);
                                    let resource_for_detail = resource.clone();
                                    let resource_for_edit = resource.clone();
                                    let resource_for_delete = resource.clone();
                                    let total_resources = get_mock_resources().into_iter().filter(|r| {
                                        r.category == selected_category() && 
                                        r.resource_type == selected_resource_type()
                                    }).count();
                                    
                                    rsx! {
                                        div {
                                            key: "{resource.id}",
                                            class: "bg-white rounded-lg shadow-sm border border-gray-200 p-4 hover:shadow-md transition-shadow",
                                            div { class: "flex items-center justify-between",
                                                div { class: "flex items-center space-x-4",
                                                    div { 
                                                        class: "w-24 h-24 flex items-center justify-center",
                                                        dangerous_inner_html: "{icon.replace(\"width=\\\"800\\\" height=\\\"800\\\"\", \"width=\\\"64\\\" height=\\\"64\\\"\").replace(\"width=\\\"88\\\" height=\\\"30\\\"\", \"width=\\\"64\\\" height=\\\"64\\\"\")}"
                                                    }
                                                    div {
                                                        div { class: "font-medium text-gray-900", "{name}" }
                                                        div { class: "text-sm text-gray-500", "{description}" }
                                                    }
                                                }
                                                div { class: "flex items-center space-x-4",
                                                    span {
                                                        class: "px-2 py-1 rounded-full text-xs font-medium {status_class}",
                                                        "{status}"
                                                    }
                                                    div { class: "relative",
                                                        button {
                                                            class: "btn btn-ghost btn-xs p-1 relative",
                                                            onclick: move |event| {
                                                                event.stop_propagation();
                                                                // 如果当前资源的菜单已经显示，则关闭；否则显示
                                                                if show_action_menu() == Some(resource.id.clone()) {
                                                                    show_action_menu.set(None);
                                                                } else {
                                                                    // 判断菜单显示位置
                                                                    let position = if index > total_resources / 2 { "top" } else { "bottom" };
                                                                    menu_position.set(position);
                                                                    
                                                                    show_action_menu.set(Some(resource.id.clone()));
                                                                }
                                                            },
                                                            svg {
                                                                class: "w-4 h-4",
                                                                fill: "none",
                                                                stroke: "currentColor",
                                                                stroke_width: "2",
                                                                view_box: "0 0 24 24",
                                                                path { d: "M4 6h16M4 12h16M4 18h16" }
                                                            }

                                                            // 操作菜单 - 根据位置显示在三点按钮上方或下方
                                                            if show_action_menu() == Some(resource.id.clone()) {
                                                                div {
                                                                    class: "absolute w-32 bg-base-100 border border-base-300 rounded-lg shadow-lg z-50",
                                                                    style: if menu_position() == "top" {
                                                                        "right: 0; bottom: 100%; margin-bottom: 0.25rem;"
                                                                    } else {
                                                                        "right: 0; top: 100%; margin-top: 0.25rem;"
                                                                    },
                                                                    div { class: "py-1",
                                                                        button {
                                                                            class: "w-full px-3 py-2 text-left text-sm hover:bg-base-200 flex items-center gap-2",
                                                                                                                                                        onclick: move |event| {
                                                                                event.stop_propagation();
                                                                                handle_resource_detail(resource_for_detail.clone());
                                                                            },
                                                                            svg {
                                                                                class: "w-3 h-3",
                                                                                fill: "none",
                                                                                stroke: "currentColor",
                                                                                stroke_width: "2",
                                                                                view_box: "0 0 24 24",
                                                                                path { d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z" }
                                                                                path { d: "M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" }
                                                                            }
                                                                            "详情"
                                                                        }
                                                                        button {
                                                                            class: "w-full px-3 py-2 text-left text-sm hover:bg-base-200 flex items-center gap-2",
                                                                            onclick: move |event| {
                                                                                event.stop_propagation();
                                                                                handle_resource_edit(resource_for_edit.clone());
                                                                            },
                                                                            svg {
                                                                                class: "w-3 h-3",
                                                                                fill: "none",
                                                                                stroke: "currentColor",
                                                                                stroke_width: "2",
                                                                                view_box: "0 0 24 24",
                                                                                path { d: "M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" }
                                                                            }
                                                                            "编辑"
                                                                        }
                                                                        button {
                                                                            class: "w-full px-3 py-2 text-left text-sm hover:bg-base-200 flex items-center gap-2 text-error",
                                                                            onclick: move |event| {
                                                                                event.stop_propagation();
                                                                                handle_resource_delete(resource_for_delete.clone());
                                                                            },
                                                                            svg {
                                                                                class: "w-3 h-3",
                                                                                fill: "none",
                                                                                stroke: "currentColor",
                                                                                stroke_width: "2",
                                                                                view_box: "0 0 24 24",
                                                                                path { d: "M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" }
                                                                            }
                                                                            "删除"
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
                                })}
                            }
                        }
                    }
                }
            }

            // 新增资源弹窗
            if show_add_dialog() {
                ResourceDialog {
                    mode: ResourceModalMode::Add,
                    on_close: move |_| {
                        show_add_dialog.set(false);
                    },
                    on_test_connection: move |_| {
                        // TODO: 测试连接
                    },
                    on_save: move |_| {
                        // TODO: 保存资源
                    }
                }
            }

            // 资源详情/编辑弹窗
            if show_resource_dialog() {
                ResourceDialog {
                    mode: resource_modal_mode(),
                    on_close: move |_| {
                        show_resource_dialog.set(false);
                    },
                    on_test_connection: move |_| {
                        // TODO: 测试连接
                    },
                    on_save: move |_| {
                        // TODO: 保存资源
                    }
                }
            }

            // 资源删除确认弹窗
            if show_delete_dialog() {
                if let Some(resource) = selected_resource_for_action() {
                    ResourceDeleteDialog {
                        resource: resource,
                        on_confirm: handle_delete_confirm,
                        on_cancel: handle_delete_cancel,
                    }
                }
            }
        }
    }
}

// 获取所有分类
fn get_all_categories() -> Vec<Category> {
    vec![
        Category::RelationalDatabase,
        Category::VectorDatabase,
        Category::Filesystem,
        Category::Queue,
        Category::BatchCompute,
        Category::StreamCompute,
    ]
}

// 模拟资源数据
fn get_mock_resources() -> Vec<Resource> {
    vec![
        Resource {
            id: "7".to_string(),
            name: "Doris分析型数据库".to_string(),
            description: "高性能MPP分析型数据库".to_string(),
            category: Category::RelationalDatabase,
            resource_type: ResourceType::Doris,
            config: serde_json::json!({}),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            status: Status::Active,
        },
        Resource {
            id: "1".to_string(),
            name: "MySQL主库".to_string(),
            description: "生产环境主数据库".to_string(),
            category: Category::RelationalDatabase,
            resource_type: ResourceType::Mysql,
            config: serde_json::json!({}),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            status: Status::Active,
        },
        Resource {
            id: "2".to_string(),
            name: "PostgreSQL从库".to_string(),
            description: "只读从数据库".to_string(),
            category: Category::RelationalDatabase,
            resource_type: ResourceType::Postgres,
            config: serde_json::json!({}),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            status: Status::Active,
        },
        Resource {
            id: "3".to_string(),
            name: "Kafka消息队列".to_string(),
            description: "实时消息处理队列".to_string(),
            category: Category::Queue,
            resource_type: ResourceType::Kafka,
            config: serde_json::json!({}),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            status: Status::Active,
        },
        Resource {
            id: "4".to_string(),
            name: "Spark批处理集群".to_string(),
            description: "大数据批处理计算集群".to_string(),
            category: Category::BatchCompute,
            resource_type: ResourceType::Spark,
            config: serde_json::json!({}),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            status: Status::Inactive,
        },
        Resource {
            id: "8".to_string(),
            name: "Flink流处理集群".to_string(),
            description: "实时流式数据处理集群".to_string(),
            category: Category::StreamCompute,
            resource_type: ResourceType::Flink,
            config: serde_json::json!({}),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            status: Status::Active,
        },
        Resource {
            id: "5".to_string(),
            name: "Milvus向量数据库".to_string(),
            description: "AI向量检索数据库".to_string(),
            category: Category::VectorDatabase,
            resource_type: ResourceType::Mailvus,
            config: serde_json::json!({}),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            status: Status::Active,
        },
        Resource {
            id: "6".to_string(),
            name: "HDFS存储集群".to_string(),
            description: "分布式文件存储系统".to_string(),
            category: Category::Filesystem,
            resource_type: ResourceType::Hdfs,
            config: serde_json::json!({}),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            status: Status::Active,
        },
    ]
}
