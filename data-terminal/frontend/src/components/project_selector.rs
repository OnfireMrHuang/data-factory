use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::utils::{cookie, request::{RequestBuilder, HttpRequest}};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub code: String,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing, default)]
    pub logo: String,
    #[serde(skip_serializing, default)]
    pub create_status: String,
    #[serde(skip_serializing, default)]
    pub create_msg: String,
    #[serde(skip_serializing, default)]
    pub created_at: String,
    #[serde(skip_serializing, default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub result: bool,
    pub msg: String,
    pub data: T,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectModalMode {
    Add,
    Edit(Project),
}

#[component]
pub fn ProjectSelector() -> Element {
    let mut projects = use_signal(|| Vec::<Project>::new());
    let mut loading = use_signal(|| false);
    let mut show_dropdown = use_signal(|| false);
    let mut selected_project = use_signal(|| None as Option<Project>);
    let mut show_project_modal = use_signal(|| false);
    let mut modal_mode = use_signal(|| ProjectModalMode::Add);
    let mut show_delete_modal = use_signal(|| false);
    let mut selected_project_for_action = use_signal(|| None as Option<Project>);
    let mut show_action_menu = use_signal(|| None as Option<String>);

    // 获取项目列表
    let fetch_projects = {
        let mut projects = projects.clone();
        let mut loading = loading.clone();
        move || {
            loading.set(true);
            spawn(async move {
                let client = crate::utils::request::create_client("http://localhost:3000");
                let req_config = RequestBuilder::new()
                    .header("Content-Type", "application/json")
                    .header("Cookie", &cookie::get_browser_cookies())
                    .build();
                let response = client.get("/api/v1/project/list", Some(req_config)).await;   
                match response {
                    Ok(response_text) => {
                        if let Ok(api_response) = serde_json::from_str::<ApiResponse<Vec<Project>>>(&response_text) {
                            if api_response.result {
                                projects.set(api_response.data);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to fetch projects: {}", e);
                    }
                }
                loading.set(false);
            });
        }
    };

    // 组件挂载时获取项目列表
    use_effect(move || {
        let mut fetch_projects = fetch_projects;
        fetch_projects();
    });

    // 处理新增项目
    let handle_add_project = {
        let mut show_project_modal = show_project_modal.clone();
        let mut modal_mode = modal_mode.clone();
        move |_| {
            modal_mode.set(ProjectModalMode::Add);
            show_project_modal.set(true);
        }
    };

    // 处理编辑项目
    let mut handle_edit_project = {
        let mut show_project_modal = show_project_modal.clone();
        let mut modal_mode = modal_mode.clone();
        move |project: Project| {
            modal_mode.set(ProjectModalMode::Edit(project));
            show_project_modal.set(true);
        }
    };

    // 处理确认操作
    let handle_confirm = {
        let mut show_project_modal = show_project_modal.clone();
        let mut fetch_projects = fetch_projects.clone();
        move |project_data: (String, String, String)| {
            let (code, name, description) = project_data;
            spawn(async move {
                let client = crate::utils::request::create_client("http://localhost:3000");
                let req_config = RequestBuilder::new()
                    .header("Content-Type", "application/json")
                    .header("Cookie", &cookie::get_browser_cookies())
                    .build();
                
                match modal_mode() {
                    ProjectModalMode::Add => {
                        // 新增项目
                        let project_data  = Project{
                            code: code,
                            name: name,
                            description: description,
                            logo: String::new(),
                            create_status: String::new(),
                            create_msg: String::new(),
                            created_at: String::new(),
                            updated_at: String::new(),
                        };
                        let response = client.post("/api/v1/project/add", Some(req_config), Some(project_data)).await;
                        match response {
                            Ok(_) => {
                                log::info!("Project add successfully");
                                fetch_projects();
                            }
                            Err(e) => {
                                log::error!("Failed to add project: {}", e);
                            }
                        }
                    }
                    ProjectModalMode::Edit(_) => {
                        // 编辑项目
                        let project_data = Project{
                            code: code,
                            name: name,
                            description: description,
                            logo: String::new(),
                            create_status: String::new(),
                            create_msg: String::new(),
                            created_at: String::new(),
                            updated_at: String::new(),
                        };
                        let response = client.post("/api/v1/project/update", Some(req_config), Some(project_data)).await;
                        match response {
                            Ok(_) => {
                                log::info!("Project updated successfully");
                                fetch_projects();
                            }
                            Err(e) => {
                                log::error!("Failed to update project: {}", e);
                            }
                        }
                    }
                }
            });
            show_project_modal.set(false);
        }
    };

    // 处理取消操作
    let handle_cancel = {
        let mut show_project_modal = show_project_modal.clone();
        move |_| {
            show_project_modal.set(false);
        }
    };



    // 处理删除确认
    let handle_delete_confirm = {
        let mut show_delete_modal = show_delete_modal.clone();
        let mut fetch_projects = fetch_projects.clone();
        move |project: Project| {
            spawn(async move {
                let client = crate::utils::request::create_client("http://localhost:3000");
                let req_config = RequestBuilder::new()
                    .header("Content-Type", "application/json")
                    .header("Cookie", &cookie::get_browser_cookies())
                    .build();
                
                let project_data = serde_json::json!({
                    "code": project.code
                });
                let response = client.post("/api/v1/project/delete", Some(req_config), Some(project_data.to_string())).await;
                match response {
                    Ok(_) => {
                        log::info!("Project deleted successfully");
                        fetch_projects();
                    }
                    Err(e) => {
                        log::error!("Failed to delete project: {}", e);
                    }
                }
            });
            show_delete_modal.set(false);
        }
    };

    // 处理删除取消
    let handle_delete_cancel = {
        let mut show_delete_modal = show_delete_modal.clone();
        move |_| {
            show_delete_modal.set(false);
        }
    };

    // 处理点击外部关闭菜单
    let handle_click_outside = {
        let mut show_action_menu = show_action_menu.clone();
        move |_| {
            show_action_menu.set(None);
        }
    };

    rsx! {
        div { 
            class: "relative",
            onclick: handle_click_outside,
            // 项目选择器按钮
            button { 
                class: "flex items-center gap-2 px-3 py-2 bg-base-200 hover:bg-base-300 rounded-lg transition-colors",
                onclick: move |_| show_dropdown.set(!show_dropdown()),
                // 项目图标
                svg { 
                    class: "w-5 h-5 text-base-content", 
                    fill: "none", 
                    stroke: "currentColor", 
                    stroke_width: "2", 
                    view_box: "0 0 24 24",
                    path { d: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" }
                }
                // 项目名称或默认文本
                span { class: "text-sm font-medium",
                    if let Some(project) = selected_project() {
                        "{project.name}"
                    } else {
                        "选择一个项目"
                    }
                }
                // 下拉箭头
                svg { 
                    class: "w-4 h-4 transition-transform", 
                    class: if show_dropdown() { "rotate-180" } else { "" },
                    fill: "none", 
                    stroke: "currentColor", 
                    stroke_width: "2", 
                    view_box: "0 0 24 24",
                    path { d: "M19 9l-7 7-7-7" }
                }
            }

            // 下拉菜单
            if show_dropdown() {
                div { 
                    class: "absolute top-full left-0 mt-2 w-80 bg-base-100 border border-base-300 rounded-lg shadow-lg z-50",
                    // 项目配置区
                    div { class: "p-4 border-b border-base-300",
                        div { class: "flex items-center justify-between",
                            h3 { class: "text-sm font-semibold text-base-content", "项目配置" }
                            button { 
                                class: "btn btn-sm btn-primary",
                                onclick: handle_add_project,
                                // 加号图标
                                svg { 
                                    class: "w-4 h-4", 
                                    fill: "none", 
                                    stroke: "currentColor", 
                                    stroke_width: "2", 
                                    view_box: "0 0 24 24",
                                    path { d: "M12 4v16m8-8H4" }
                                }
                                "新增项目"
                            }
                        }
                    }

                    // 项目列表区
                    div { class: "max-h-64 overflow-y-auto",
                        if loading() {
                            div { class: "p-4 text-center text-base-content/60",
                                "加载中..."
                            }
                        } else {
                            if projects().is_empty() {
                                div { class: "p-4 text-center text-base-content/60",
                                    "暂无项目"
                                }
                            } else {
                                div { class: "space-y-1",
                                    for project in projects() {
                                        let project_clone = project.clone();
                                        div { 
                                            class: "flex items-center justify-between p-3 hover:bg-base-200 cursor-pointer",
                                            onclick: move |_| {
                                                selected_project.set(Some(project_clone.clone()));
                                                show_dropdown.set(false);
                                            },
                                            div { class: "flex items-center gap-3",
                                                div { class: "w-8 h-8 bg-primary/10 rounded-lg flex items-center justify-center",
                                                    svg { 
                                                        class: "w-4 h-4 text-primary", 
                                                        fill: "none", 
                                                        stroke: "currentColor", 
                                                        stroke_width: "2", 
                                                        view_box: "0 0 24 24",
                                                        path { d: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" }
                                                    }
                                                }
                                                div { class: "flex-1",
                                                    div { class: "font-medium text-sm", "{project.name}" }
                                                    div { class: "text-xs text-base-content/60", "{project.description}" }
                                                }
                                            }
                                            div { class: "relative",
                                                button { 
                                                    class: "btn btn-ghost btn-xs",
                                                    onclick: move |event| {
                                                        event.stop_propagation();
                                                        selected_project_for_action.set(Some(project.clone()));
                                                        show_action_menu.set(Some(project.code.clone()));
                                                    },
                                                    svg { 
                                                        class: "w-3 h-3", 
                                                        fill: "none", 
                                                        stroke: "currentColor", 
                                                        stroke_width: "2", 
                                                        view_box: "0 0 24 24",
                                                        path { d: "M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" }
                                                    }
                                                }
                                                
                                                // 操作菜单
                                                if show_action_menu() == Some(project.code.clone()) {
                                                    div { 
                                                        class: "absolute right-0 top-full mt-1 w-32 bg-base-100 border border-base-300 rounded-lg shadow-lg z-50",
                                                        div { class: "py-1",
                                                            button { 
                                                                class: "w-full px-3 py-2 text-left text-sm hover:bg-base-200 flex items-center gap-2",
                                                                onclick: move |_| {
                                                                    modal_mode.set(ProjectModalMode::Edit(project.clone()));
                                                                    show_project_modal.set(true);
                                                                    show_action_menu.set(None);
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
                                                                onclick: move |_| {
                                                                    selected_project_for_action.set(Some(project.clone()));
                                                                    show_delete_modal.set(true);
                                                                    show_action_menu.set(None);
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
                    }
                }
            }
        }

        // 项目新增/编辑弹窗
        ProjectAddOrEdit {
            show: show_project_modal(),
            mode: modal_mode(),
            on_confirm: handle_confirm,
            on_cancel: handle_cancel,
        }

        // 项目删除确认弹窗
        if show_delete_modal() {
            if let Some(project) = selected_project_for_action() {
                ProjectDel {
                    project: project,
                    on_confirm: handle_delete_confirm,
                    on_cancel: handle_delete_cancel,
                }
            }
        }
    }
} 

#[component]
pub fn ProjectAddOrEdit(
    show: bool,
    mode: ProjectModalMode,
    on_confirm: Callback<(String, String, String)>,
    on_cancel: Callback<()>,
) -> Element {
    let mut project_code = use_signal(|| {
        match &mode {
            ProjectModalMode::Add => String::new(),
            ProjectModalMode::Edit(project) => project.code.clone(),
        }
    });
    let mut project_name = use_signal(|| {
        match &mode {
            ProjectModalMode::Add => String::new(),
            ProjectModalMode::Edit(project) => project.name.clone(),
        }
    });
    let mut project_description = use_signal(|| {
        match &mode {
            ProjectModalMode::Add => String::new(),
            ProjectModalMode::Edit(project) => project.description.clone(),
        }
    });

    let is_edit_mode = matches!(mode, ProjectModalMode::Edit(_));
    let modal_title = if is_edit_mode { "编辑项目" } else { "添加新项目" };

    let handle_confirm = {
        let on_confirm = on_confirm.clone();
        move |_| {
            on_confirm.call((project_code(), project_name(), project_description()));
        }
    };

    let handle_cancel = {
        let on_cancel = on_cancel.clone();
        move |_| {
            on_cancel.call(());
        }
    };

    rsx! {
        dialog { 
            class: if show { "modal modal-open" } else { "modal" },
            div { 
                class: "modal-box",
                h3 { 
                    class: "text-xl font-bold mb-6", 
                    "{modal_title}" 
                }
                
                form { 
                    class: "space-y-4",
                    
                    // 项目编码输入框（仅在新增模式下显示）
                    if !is_edit_mode {
                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text font-medium", "项目编码" }
                            }
                            input { 
                                class: "input input-bordered w-full",
                                placeholder: "请输入项目编码",
                                value: "{project_code}",
                                oninput: move |event| project_code.set(event.value().to_string())
                            }
                        }
                    }
                    
                    div { class: "form-control",
                        label { class: "label",
                            span { class: "label-text font-medium", "项目名称" }
                        }
                        input { 
                            class: "input input-bordered w-full",
                            placeholder: "请输入项目名称",
                            value: "{project_name}",
                            oninput: move |event| project_name.set(event.value().to_string())
                        }
                    }
                    
                    div { class: "form-control",
                        label { class: "label",
                            span { class: "label-text font-medium", "项目描述" }
                        }
                        textarea { 
                            class: "textarea textarea-bordered w-full h-24",
                            placeholder: "请输入项目描述",
                            value: "{project_description}",
                            oninput: move |event| project_description.set(event.value()),
                        }
                    }
                }
                
                div { 
                    class: "modal-action",
                    button { 
                        class: "btn btn-outline",
                        onclick: handle_cancel,
                        "取消" 
                    }
                    
                    button { 
                        class: "btn btn-primary",
                        onclick: handle_confirm,
                        if is_edit_mode { "更新" } else { "确认" }
                    }
                }
            }
            
            form { 
                method: "dialog", 
                class: "modal-backdrop",
                onclick: handle_cancel,
                button { "close" }
            }
        }
    }
}

#[component]
pub fn ProjectDel(
    project: Project,
    on_confirm: Callback<Project>,
    on_cancel: Callback<()>,
) -> Element {
    let handle_confirm = {
        let on_confirm = on_confirm.clone();
        let project = project.clone();
        move |_| {
            on_confirm.call(project.clone());
        }
    };

    let handle_cancel = {
        let on_cancel = on_cancel.clone();
        move |_| {
            on_cancel.call(());
        }
    };

    rsx! {
        dialog { 
            class: "modal modal-open",
            div { 
                class: "modal-box",
                h3 { 
                    class: "text-lg font-bold mb-4", 
                    "删除确认" 
                }
                
                p { 
                    class: "py-4 text-base-content/80", 
                    "请确认是否删除该项目" 
                }
                
                div { 
                    class: "modal-action",
                    button { 
                        class: "btn btn-outline",
                        onclick: handle_cancel,
                        "取消" 
                    }
                    
                    button { 
                        class: "btn btn-error",
                        onclick: handle_confirm,
                        "确认删除" 
                    }
                }
            }
            
            form { 
                method: "dialog", 
                class: "modal-backdrop",
                onclick: handle_cancel,
                button { "close" }
            }
        }
    }
}