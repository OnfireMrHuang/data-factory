use dioxus::prelude::*;
use crate::models::protocol::{
    Category, ResourceType, Resource,
    DatabaseConfigForm, QueueConfigForm, FileSystemConfigForm, 
    VectorDatabaseConfigForm, BatchComputeConfigForm, StreamComputeConfigForm
};

#[derive(Clone, PartialEq)]
pub enum ResourceModalMode {
    Add,
    Edit(Resource),
    Detail(Resource),
}

#[component]
pub fn ResourceDialog(
    mode: ResourceModalMode,
    on_close: EventHandler<()>,
    on_test_connection: EventHandler<()>,
    on_save: EventHandler<()>,
) -> Element {
    let is_detail_mode = matches!(mode, ResourceModalMode::Detail(_));
    let is_edit_mode = matches!(mode, ResourceModalMode::Edit(_));
    
    // 初始化表单数据
    let mut dialog_category = use_signal(|| {
        match &mode {
            ResourceModalMode::Add => Category::RelationalDatabase,
            ResourceModalMode::Edit(resource) | ResourceModalMode::Detail(resource) => resource.category.clone(),
        }
    });
    
    let mut dialog_resource_type = use_signal(|| {
        match &mode {
            ResourceModalMode::Add => ResourceType::Mysql,
            ResourceModalMode::Edit(resource) | ResourceModalMode::Detail(resource) => resource.resource_type.clone(),
        }
    });
    
    let mut resource_name = use_signal(|| {
        match &mode {
            ResourceModalMode::Add => String::new(),
            ResourceModalMode::Edit(resource) | ResourceModalMode::Detail(resource) => resource.name.clone(),
        }
    });
    
    let mut resource_description = use_signal(|| {
        match &mode {
            ResourceModalMode::Add => String::new(),
            ResourceModalMode::Edit(resource) | ResourceModalMode::Detail(resource) => resource.description.clone(),
        }
    });
    
    // 配置表单状态
    let mut database_config = use_signal(|| DatabaseConfigForm {
        host: String::new(),
        port: 3306,
        username: String::new(),
        password: String::new(),
        databases: vec![],
    });
    
    let mut queue_config = use_signal(|| QueueConfigForm {
        host: String::new(),
        port: 9092,
        admin_port: 8080,
        username: None,
        password: None,
        virtual_host: None,
        cluster_name: None,
        ssl_enabled: false,
        sasl_enabled: false,
        sasl_mechanism: None,
    });
    
    let mut filesystem_config = use_signal(|| FileSystemConfigForm {
        host: String::new(),
        port: 9000,
        username: None,
        password: None,
        ssl_enabled: false,
        auth_token: None,
        access_key_id: None,
        secret_access_key: None,
        region: None,
        bucket: None,
    });
    
    let mut vector_config = use_signal(|| VectorDatabaseConfigForm {
        host: String::new(),
        port: 19530,
        username: None,
        password: None,
        ssl_enabled: false,
        collection_name: None,
        dimension: None,
        metric_type: None,
    });
    
    let mut batch_compute_config = use_signal(|| BatchComputeConfigForm {
        host: String::new(),
        port: 7077,
        username: None,
        password: None,
        ssl_enabled: false,
        cluster_name: None,
        master_url: None,
        worker_nodes: None,
    });
    
    let mut stream_compute_config = use_signal(|| StreamComputeConfigForm {
        host: String::new(),
        port: 8081,
        username: None,
        password: None,
        ssl_enabled: false,
        cluster_name: None,
        job_manager_url: None,
        task_manager_count: None,
    });

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

    // 获取所有分类
    let get_all_categories = || {
        vec![
            Category::RelationalDatabase,
            Category::VectorDatabase,
            Category::Filesystem,
            Category::Queue,
            Category::BatchCompute,
            Category::StreamCompute,
        ]
    };

    // 渲染配置表单
    let render_config_form = |category: &Category| {
        match category {
            Category::RelationalDatabase => {
                rsx! {
                    div { class: "space-y-4",
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "主机地址" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{database_config().host}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            database_config.set(DatabaseConfigForm {
                                                host: event.value(),
                                                ..database_config()
                                            });
                                        }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "端口" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "number",
                                    disabled: is_detail_mode,
                                    value: "{database_config().port}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            if let Ok(port) = event.value().parse::<u16>() {
                                                database_config.set(DatabaseConfigForm {
                                                    port,
                                                    ..database_config()
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "用户名" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{database_config().username}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            database_config.set(DatabaseConfigForm {
                                                username: event.value(),
                                                ..database_config()
                                            });
                                        }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "密码" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "password",
                                    disabled: is_detail_mode,
                                    value: "{database_config().password}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            database_config.set(DatabaseConfigForm {
                                                password: event.value(),
                                                ..database_config()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Category::Queue => {
                rsx! {
                    div { class: "space-y-4",
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "主机地址" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{queue_config().host}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            queue_config.set(QueueConfigForm {
                                                host: event.value(),
                                                ..queue_config()
                                            });
                                        }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "端口" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "number",
                                    disabled: is_detail_mode,
                                    value: "{queue_config().port}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            if let Ok(port) = event.value().parse::<u16>() {
                                                queue_config.set(QueueConfigForm {
                                                    port,
                                                    ..queue_config()
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "管理端口" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "number",
                                    disabled: is_detail_mode,
                                    value: "{queue_config().admin_port}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            if let Ok(port) = event.value().parse::<u16>() {
                                                queue_config.set(QueueConfigForm {
                                                    admin_port: port,
                                                    ..queue_config()
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "集群名称" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{queue_config().cluster_name.as_deref().unwrap_or(\"\")}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            queue_config.set(QueueConfigForm {
                                                cluster_name: Some(event.value()),
                                                ..queue_config()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "flex items-center space-x-4",
                            label { class: "label cursor-pointer",
                                input {
                                    class: "checkbox",
                                    r#type: "checkbox",
                                    disabled: is_detail_mode,
                                    checked: queue_config().ssl_enabled,
                                    onchange: move |event| {
                                        if !is_detail_mode {
                                            queue_config.set(QueueConfigForm {
                                                ssl_enabled: event.checked(),
                                                ..queue_config()
                                            });
                                        }
                                    }
                                }
                                span { class: "label-text ml-2", "启用SSL" }
                            }
                            label { class: "label cursor-pointer",
                                input {
                                    class: "checkbox",
                                    r#type: "checkbox",
                                    disabled: is_detail_mode,
                                    checked: queue_config().sasl_enabled,
                                    onchange: move |event| {
                                        if !is_detail_mode {
                                            queue_config.set(QueueConfigForm {
                                                sasl_enabled: event.checked(),
                                                ..queue_config()
                                            });
                                        }
                                    }
                                }
                                span { class: "label-text ml-2", "启用SASL" }
                            }
                        }
                    }
                }
            }
            Category::Filesystem => {
                rsx! {
                    div { class: "space-y-4",
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "主机地址" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{filesystem_config().host}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            filesystem_config.set(FileSystemConfigForm {
                                                host: event.value(),
                                                ..filesystem_config()
                                            });
                                        }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "端口" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "number",
                                    disabled: is_detail_mode,
                                    value: "{filesystem_config().port}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            if let Ok(port) = event.value().parse::<u16>() {
                                                filesystem_config.set(FileSystemConfigForm {
                                                    port,
                                                    ..filesystem_config()
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "访问密钥ID" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{filesystem_config().access_key_id.as_deref().unwrap_or(\"\")}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            filesystem_config.set(FileSystemConfigForm {
                                                access_key_id: Some(event.value()),
                                                ..filesystem_config()
                                            });
                                        }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "密钥" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "password",
                                    disabled: is_detail_mode,
                                    value: "{filesystem_config().secret_access_key.as_deref().unwrap_or(\"\")}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            filesystem_config.set(FileSystemConfigForm {
                                                secret_access_key: Some(event.value()),
                                                ..filesystem_config()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "区域" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{filesystem_config().region.as_deref().unwrap_or(\"\")}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            filesystem_config.set(FileSystemConfigForm {
                                                region: Some(event.value()),
                                                ..filesystem_config()
                                            });
                                        }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "存储桶" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{filesystem_config().bucket.as_deref().unwrap_or(\"\")}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            filesystem_config.set(FileSystemConfigForm {
                                                bucket: Some(event.value()),
                                                ..filesystem_config()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Category::VectorDatabase => {
                rsx! {
                    div { class: "space-y-4",
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "主机地址" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{vector_config().host}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            vector_config.set(VectorDatabaseConfigForm {
                                                host: event.value(),
                                                ..vector_config()
                                            });
                                        }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "端口" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "number",
                                    disabled: is_detail_mode,
                                    value: "{vector_config().port}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            if let Ok(port) = event.value().parse::<u16>() {
                                                vector_config.set(VectorDatabaseConfigForm {
                                                    port,
                                                    ..vector_config()
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "集合名称" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{vector_config().collection_name.as_deref().unwrap_or(\"\")}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            vector_config.set(VectorDatabaseConfigForm {
                                                collection_name: Some(event.value()),
                                                ..vector_config()
                                            });
                                        }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "向量维度" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "number",
                                    disabled: is_detail_mode,
                                    value: "{vector_config().dimension.unwrap_or(0)}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            if let Ok(dimension) = event.value().parse::<u32>() {
                                                vector_config.set(VectorDatabaseConfigForm {
                                                    dimension: Some(dimension),
                                                    ..vector_config()
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Category::BatchCompute => {
                rsx! {
                    div { class: "space-y-4",
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "主机地址" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{batch_compute_config().host}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            batch_compute_config.set(BatchComputeConfigForm {
                                                host: event.value(),
                                                ..batch_compute_config()
                                            });
                                        }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "端口" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "number",
                                    disabled: is_detail_mode,
                                    value: "{batch_compute_config().port}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            if let Ok(port) = event.value().parse::<u16>() {
                                                batch_compute_config.set(BatchComputeConfigForm {
                                                    port,
                                                    ..batch_compute_config()
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "集群名称" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{batch_compute_config().cluster_name.as_deref().unwrap_or(\"\")}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            batch_compute_config.set(BatchComputeConfigForm {
                                                cluster_name: Some(event.value()),
                                                ..batch_compute_config()
                                            });
                                        }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Master URL" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{batch_compute_config().master_url.as_deref().unwrap_or(\"\")}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            batch_compute_config.set(BatchComputeConfigForm {
                                                master_url: Some(event.value()),
                                                ..batch_compute_config()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Category::StreamCompute => {
                rsx! {
                    div { class: "space-y-4",
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "主机地址" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{stream_compute_config().host}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            stream_compute_config.set(StreamComputeConfigForm {
                                                host: event.value(),
                                                ..stream_compute_config()
                                            });
                                        }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "端口" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "number",
                                    disabled: is_detail_mode,
                                    value: "{stream_compute_config().port}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            if let Ok(port) = event.value().parse::<u16>() {
                                                stream_compute_config.set(StreamComputeConfigForm {
                                                    port,
                                                    ..stream_compute_config()
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "集群名称" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{stream_compute_config().cluster_name.as_deref().unwrap_or(\"\")}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            stream_compute_config.set(StreamComputeConfigForm {
                                                cluster_name: Some(event.value()),
                                                ..stream_compute_config()
                                            });
                                        }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Job Manager URL" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    value: "{stream_compute_config().job_manager_url.as_deref().unwrap_or(\"\")}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            stream_compute_config.set(StreamComputeConfigForm {
                                                job_manager_url: Some(event.value()),
                                                ..stream_compute_config()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                rsx! {
                    div { class: "text-base-content/60 text-center py-4", "暂不支持此类型的资源配置" }
                }
            }
        }
    };

    let modal_title = match mode {
        ResourceModalMode::Add => "新增资源",
        ResourceModalMode::Edit(_) => "编辑资源",
        ResourceModalMode::Detail(_) => "资源详情",
    };

    rsx! {
        dialog {
            class: "modal modal-open",
            div {
                class: "modal-box w-11/12 max-w-4xl max-h-[90vh] overflow-y-auto",
                h3 {
                    class: "text-xl font-bold mb-6",
                    "{modal_title}"
                }

                div { class: "space-y-6",
                    // 资源分类选择
                    div { class: "form-control",
                        label { class: "label",
                            span { class: "label-text font-medium", "资源分类" }
                        }
                        select {
                            class: "select select-bordered w-full",
                            disabled: is_detail_mode || is_edit_mode,
                            value: "{dialog_category:?}",
                            onchange: move |event| {
                                if !is_detail_mode && !is_edit_mode {
                                    if let Ok(category) = serde_json::from_str::<Category>(&event.value()) {
                                        dialog_category.set(category.clone());
                                        if let Some(first_type) = get_resource_types(category).first() {
                                            dialog_resource_type.set(first_type.clone());
                                        }
                                    }
                                }
                            },
                            {get_all_categories().into_iter().map(|category| {
                                rsx! {
                                    option {
                                        key: "{category:?}",
                                        value: "{category:?}",
                                        "{get_category_name(&category)}"
                                    }
                                }
                            })}
                        }
                    }

                    // 资源类型选择
                    div { class: "form-control",
                        label { class: "label",
                            span { class: "label-text font-medium", "资源类型" }
                        }
                        select {
                            class: "select select-bordered w-full",
                            disabled: is_detail_mode || is_edit_mode,
                            value: "{dialog_resource_type:?}",
                            onchange: move |event| {
                                if !is_detail_mode && !is_edit_mode {
                                    if let Ok(resource_type) = serde_json::from_str::<ResourceType>(&event.value()) {
                                        dialog_resource_type.set(resource_type);
                                    }
                                }
                            },
                            {get_resource_types(dialog_category()).into_iter().map(|resource_type| {
                                rsx! {
                                    option {
                                        key: "{resource_type:?}",
                                        value: "{resource_type:?}",
                                        "{get_resource_type_name(&resource_type)}"
                                    }
                                }
                            })}
                        }
                    }

                    // 基本信息
                    div { class: "space-y-4",
                        h4 { class: "text-lg font-semibold", "基本信息" }
                        div { class: "grid grid-cols-1 gap-4",
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "资源名称" }
                                }
                                input {
                                    class: "input input-bordered w-full",
                                    disabled: is_detail_mode,
                                    placeholder: "请输入资源名称",
                                    value: "{resource_name()}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            resource_name.set(event.value());
                                        }
                                    }
                                }
                            }
                            div { class: "form-control",
                                label { class: "label",
                                    span { class: "label-text font-medium", "资源描述" }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full h-24",
                                    disabled: is_detail_mode,
                                    placeholder: "请输入资源描述",
                                    value: "{resource_description()}",
                                    oninput: move |event| {
                                        if !is_detail_mode {
                                            resource_description.set(event.value());
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 资源配置
                    div { class: "space-y-4",
                        h4 { class: "text-lg font-semibold", "资源配置" }
                        {render_config_form(&dialog_category())}
                    }
                }

                div {
                    class: "modal-action",
                    button {
                        class: "btn btn-outline",
                        onclick: move |_| {
                            on_close.call(());
                        },
                        "关闭"
                    }
                    if !is_detail_mode {
                        button {
                            class: "btn btn-info",
                            onclick: move |_| {
                                on_test_connection.call(());
                            },
                            "测试连接"
                        }
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| {
                                on_save.call(());
                            },
                            if is_edit_mode { "更新" } else { "保存" }
                        }
                    }
                }
            }

            form {
                method: "dialog",
                class: "modal-backdrop",
                onclick: move |_| {
                    on_close.call(());
                },
                button { "close" }
            }
        }
    }
} 