use dioxus::prelude::*;
use crate::models::protocol::{Resource, Category, ResourceType, Status};

#[component]
pub fn ResourcePage() -> Element {
    let mut selected_category = use_signal(|| Category::RelationalDatabase);
    let mut selected_resource_type = use_signal(|| ResourceType::Mysql);

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
            ResourceType::Mysql => "🗄️",
            ResourceType::Postgres => "🐘",
            ResourceType::Doris => "🦌",
            ResourceType::Mailvus => "🔍",
            ResourceType::Spark => "⚡",
            ResourceType::Flink => "🌊",
            ResourceType::Kafka => "📨",
            ResourceType::Hdfs => "📁",
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

    rsx! {
        div { class: "flex flex-col h-screen bg-gray-50",
            // 导航栏
            div { class: "bg-white shadow-sm border-b border-gray-200 px-6 py-4 flex justify-between items-center",
                div { class: "flex items-center space-x-4",
                    button {
                        class: "text-gray-600 hover:text-gray-900 flex items-center space-x-2",
                        onclick: move |_| {
                            // TODO: 导航到Home页面
                        },
                        span { "←" }
                        span { "返回Home" }
                    }
                }
                div { class: "flex items-center space-x-4",
                    button {
                        class: "bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 flex items-center space-x-2",
                        onclick: move |_| {
                            // TODO: 打开新增资源对话框
                        },
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
                                }).map(|resource| {
                                    let status_class = get_status_color(&resource.status);
                                    let icon = get_resource_icon(&resource.resource_type);
                                    let name = resource.name.clone();
                                    let description = resource.description.clone();
                                    let status = format!("{:?}", resource.status);
                                    rsx! {
                                        div {
                                            key: "{resource.id}",
                                            class: "bg-white rounded-lg shadow-sm border border-gray-200 p-4 hover:shadow-md transition-shadow",
                                            div { class: "flex items-center justify-between",
                                                div { class: "flex items-center space-x-4",
                                                    div { class: "text-2xl", "{icon}" }
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
                                                            class: "text-gray-400 hover:text-gray-600 p-1",
                                                            onclick: move |_| {
                                                                // TODO: 显示更多操作菜单
                                                            },
                                                            "⋮"
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
