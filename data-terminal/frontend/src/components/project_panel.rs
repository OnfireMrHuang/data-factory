use crate::models::project::{Project, ProjectModalMode};
use crate::models::protocol::ApiResponse;
use crate::utils::{
    cookie,
    request::{HttpRequest, RequestBuilder},
};
use dioxus::prelude::*;
use dioxus_toast::{Icon, ToastInfo, ToastManager};

#[component]
pub fn ProjectPanel() -> Element {
    let mut projects = use_signal(|| Vec::<Project>::new()); // 项目列表
    let mut loading = use_signal(|| false); // 表示加载后端数据
    let mut show_dropdown = use_signal(|| false); // 表示下拉菜单是否显示 
    let mut selected_project = use_signal(|| None as Option<Project>); // 表示选中的项目

    let mut show_project_add_or_edit_modal = use_signal(|| false); // 表示项目新增/编辑弹窗是否显示
    let mut modal_mode = use_signal(|| ProjectModalMode::Add); // 表示项目新增/编辑弹窗的类型

    let mut show_project_delete_modal = use_signal(|| false); // 表示项目删除确认弹窗是否显示
    let mut selected_project_for_action = use_signal(|| None as Option<Project>); // 表示选中的项目
    let mut show_action_menu = use_signal(|| None as Option<String>); // 表示操作菜单是否显示
    let mut menu_position = use_signal(|| "bottom" as &str); // 表示菜单显示位置：bottom 或 top

    let mut toast = use_signal(|| ToastManager::default()); // 表示错误弹窗

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
                let mut error_msg = String::new();
                match response {
                    Ok(response_text) => {
                        let result = serde_json::from_str::<ApiResponse<Vec<Project>>>(&response_text);
                        match result {
                            Ok(api_response) => {
                                if api_response.result {
                                    projects.set(api_response.data);
                                } else {
                                    error_msg = api_response.msg;
                                }
                            }
                            Err(e) => {
                                error_msg = e.to_string();
                            }
                        }
                    }
                    Err(e) => {
                        error_msg = e.to_string();
                    }
                }
                loading.set(false);
                if !error_msg.is_empty() {
                    let _id = toast.write().popup(ToastInfo {
                        heading: Some("获取项目列表失败".into()),
                        context: error_msg,
                        allow_toast_close: true,
                        position: dioxus_toast::Position::TopRight,
                        icon: Some(Icon::Error),
                        hide_after: Some(3),
                    });
                }
            });
        }
    };

    // 组件挂载时获取项目列表
    use_effect(move || {
        let mut fetch_projects = fetch_projects;
        fetch_projects();
    });

    // 处理项目添加或编辑确认
    let handle_project_add_or_edit_confirm = {
        let mut show_project_add_or_edit_modal = show_project_add_or_edit_modal.clone();
        let mut fetch_projects = fetch_projects.clone();
        move |project_data: (String, String, String)| {
            let (code, name, description) = project_data;
            spawn(async move {
                let client = crate::utils::request::create_client("http://localhost:3000");
                let req_config = RequestBuilder::new()
                    .header("Content-Type", "application/json")
                    .header("Cookie", &cookie::get_browser_cookies())
                    .build();
                let project_data = Project {
                    code: code,
                    name: name,
                    description: description,
                    logo: String::new(),
                    create_status: String::new(),
                    create_msg: String::new(),
                    created_at: String::new(),
                    updated_at: String::new(),
                };
                let mut error_msg = String::new();
                match modal_mode() {
                    ProjectModalMode::Add => {
                        // 新增项目
                        let response = client
                            .post("/api/v1/project/add", Some(req_config), Some(project_data))
                            .await;
                        match response {
                            Ok(result) => {
                                match serde_json::from_str::<ApiResponse<String>>(&result) {
                                    Ok(api_response) => {
                                        if api_response.result {
                                            fetch_projects(); // 刷新项目列表
                                        } else {
                                            error_msg = api_response.msg;
                                        }
                                    }
                                    Err(e) => {
                                        error_msg = e.to_string();
                                    }
                                }
                            }
                            Err(e) => {
                                error_msg = e.to_string();
                            }
                        }
                        if !error_msg.is_empty() {
                            let _id = toast.write().popup(ToastInfo {
                                heading: Some("新增项目失败".into()),
                                context: error_msg,
                                allow_toast_close: true,
                                position: dioxus_toast::Position::TopRight,
                                icon: Some(Icon::Error),
                                hide_after: Some(3),
                            });
                        }
                    }
                    ProjectModalMode::Edit(_) => {
                        // 编辑项目
                        let response = client
                            .post(
                                "/api/v1/project/update",
                                Some(req_config),
                                Some(project_data),
                            )
                            .await;
                        match response {
                            Ok(result) => {
                                match serde_json::from_str::<ApiResponse<String>>(&result) {
                                    Ok(api_response) => {
                                        if api_response.result {
                                            fetch_projects(); // 刷新项目列表
                                        } else {
                                            error_msg = api_response.msg;
                                        }
                                    }
                                    Err(e) => {
                                        error_msg = e.to_string();
                                    }
                                }
                            }
                            Err(e) => {
                                error_msg = e.to_string();
                            }
                        }
                        if !error_msg.is_empty() {
                            let _id = toast.write().popup(ToastInfo {
                                heading: Some("编辑项目失败".into()),
                                context: error_msg,
                                allow_toast_close: true,
                                position: dioxus_toast::Position::TopRight,
                                icon: Some(Icon::Error),
                                hide_after: Some(3),
                            });
                        }
                    }
                }
            });
            show_project_add_or_edit_modal.set(false);
        }
    };

    // 处理取消操作
    let handle_project_add_or_edit_cancel = {
        let mut show_project_add_or_edit_modal = show_project_add_or_edit_modal.clone();
        move |_| {
            show_project_add_or_edit_modal.set(false);
        }
    };

    // 处理删除确认
    let handle_delete_confirm = {
        let mut show_project_delete_modal = show_project_delete_modal.clone();
        let mut fetch_projects = fetch_projects.clone();
        move |project: Project| {
            spawn(async move {
                let mut error_msg = String::new();
                let client = crate::utils::request::create_client("http://localhost:3000");
                let req_config = RequestBuilder::new()
                    .header("Accept", "application/json")
                    .header("Cookie", &cookie::get_browser_cookies())
                    .build();

                let response = client
                    .delete(
                        &format!("/api/v1/project/{}", project.code),
                        Some(req_config),
                    )
                    .await;
                match response {
                    Ok(result) => {
                        match serde_json::from_str::<ApiResponse<String>>(&result) {
                            Ok(api_response) => {
                                if api_response.result {
                                    fetch_projects(); // 刷新项目列表
                                } else {
                                    error_msg = api_response.msg;
                                }
                            }
                            Err(e) => {
                                error_msg = e.to_string();
                            }
                        }
                    }
                    Err(e) => {
                        error_msg = e.to_string();
                    }
                }
                if !error_msg.is_empty() {
                    let _id = toast.write().popup(ToastInfo {
                        heading: Some("删除项目失败".into()),
                        context: error_msg,
                        allow_toast_close: true,
                        position: dioxus_toast::Position::TopRight,
                        icon: Some(Icon::Error),
                        hide_after: Some(3),
                    });
                }
            });
            show_project_delete_modal.set(false);
        }
    };

    // 处理删除取消
    let handle_delete_cancel = {
        let mut show_project_delete_modal = show_project_delete_modal.clone();
        move |_| {
            show_project_delete_modal.set(false);
        }
    };

    // 处理点击外部关闭菜单
    let handle_click_outside = {
        let mut show_action_menu = show_action_menu.clone();
        move |_| {
            show_action_menu.set(None);
        }
    };

    // 处理新增项目
    let handle_add_project = {
        let mut show_project_add_or_edit_modal = show_project_add_or_edit_modal.clone();
        let mut modal_mode = modal_mode.clone();
        move |_| {
            modal_mode.set(ProjectModalMode::Add);
            show_project_add_or_edit_modal.set(true);
        }
    };

    rsx! {

        // 错误弹窗
        dioxus_toast::ToastFrame {
            manager: toast
        }

        // 项目主面板
        div {
            class: "relative",
            style: "overflow: visible;",
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
                    style: "min-width: 320px; max-width: calc(100vw - 2rem); max-height: 64rem;",
                    
                    // 项目配置区
                    ProjectConfig {
                        on_add_project: handle_add_project,
                    }

                    // 项目列表区
                    ProjectList {
                        projects: projects.clone(),
                        loading: loading.clone(),
                        selected_project: selected_project.clone(),
                        show_dropdown: show_dropdown.clone(),
                        selected_project_for_action: selected_project_for_action.clone(),
                        show_action_menu: show_action_menu.clone(),
                        modal_mode: modal_mode.clone(),
                        show_project_add_or_edit_modal: show_project_add_or_edit_modal.clone(),
                        show_project_delete_modal: show_project_delete_modal.clone(),
                        menu_position: menu_position.clone(),
                    }
                }
            }


        }

        // 项目新增/编辑弹窗
        if show_project_add_or_edit_modal() {
            ProjectAddOrEdit {
                mode: modal_mode(),
                on_confirm: handle_project_add_or_edit_confirm,
                on_cancel: handle_project_add_or_edit_cancel,
            }
        }

        // 项目删除确认弹窗
        if show_project_delete_modal() {
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

// 项目配置区组件
#[component]
pub fn ProjectConfig(
    on_add_project: Callback<()>,
) -> Element {
    rsx! {
        div { class: "p-4 border-b border-base-300 flex justify-center",
            button {
                class: "btn btn-sm btn-primary flex items-center gap-2",
                onclick: move |_| on_add_project.call(()),
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
}

// 项目列表项组件
#[component]
pub fn ProjectItem(
    index: usize,
    project: Project,
    total_projects: usize,
    selected_project: Signal<Option<Project>>,
    show_dropdown: Signal<bool>,
    selected_project_for_action: Signal<Option<Project>>,
    show_action_menu: Signal<Option<String>>,
    menu_position: Signal<&'static str>,
    modal_mode: Signal<ProjectModalMode>,
    show_project_add_or_edit_modal: Signal<bool>,
    show_project_delete_modal: Signal<bool>,
) -> Element {

    let project_for_selected = project.clone();
    let project_for_action = project.clone();
    let project_for_edit = project.clone();
    let project_for_delete = project.clone();

    rsx! {
        div {
            class: "flex items-center justify-between p-3 hover:bg-base-200 cursor-pointer relative",
            onclick: move |_| {
                selected_project.set(Some(project_for_selected.clone()));
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
            
            // 水平三点按钮
            button {
                class: "btn btn-ghost btn-xs p-1 relative",
                onclick: move |event| {
                    event.stop_propagation();
                    // 如果当前项目的菜单已经显示，则关闭；否则显示
                    if show_action_menu() == Some(project_for_action.code.clone()) {
                        show_action_menu.set(None);
                    } else {
                        // 判断菜单显示位置
                        let position = if index > total_projects / 2 { "top" } else { "bottom" };
                        menu_position.set(position);
                        
                        selected_project_for_action.set(Some(project_for_action.clone()));
                        show_action_menu.set(Some(project_for_action.code.clone()));
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
                if show_action_menu() == Some(project.code.clone()) {
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
                                    show_action_menu.set(None);
                                    modal_mode.set(ProjectModalMode::Edit(project_for_edit.clone()));
                                    show_project_add_or_edit_modal.set(true);
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
                                    show_action_menu.set(None);
                                    selected_project_for_action.set(Some(project_for_delete.clone()));
                                    show_project_delete_modal.set(true);
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

// 项目列表区组件
#[component]
pub fn ProjectList(
    projects: Signal<Vec<Project>>,
    loading: Signal<bool>,
    selected_project: Signal<Option<Project>>,
    show_dropdown: Signal<bool>,
    selected_project_for_action: Signal<Option<Project>>,
    show_action_menu: Signal<Option<String>>,
    modal_mode: Signal<ProjectModalMode>,
    show_project_add_or_edit_modal: Signal<bool>,
    show_project_delete_modal: Signal<bool>,
    menu_position: Signal<&'static str>,
) -> Element {
    let is_empty = projects().is_empty();
    let total_projects = projects().len();

    rsx! {
        div { class: "h-96 overflow-y-auto",
            if loading() {
                div { class: "p-4 text-center text-base-content/60",
                    "加载中..."
                }
            } else {
                if is_empty {
                    div { class: "p-4 text-center text-base-content/60",
                        "暂无项目"
                    }
                } else {
                    div { class: "space-y-1",
                        for (index, project) in projects().iter().enumerate() {
                            ProjectItem {
                                index: index,
                                project: project.clone(),
                                total_projects: total_projects,
                                selected_project: selected_project.clone(),
                                show_dropdown: show_dropdown.clone(),
                                selected_project_for_action: selected_project_for_action.clone(),
                                show_action_menu: show_action_menu.clone(),
                                menu_position: menu_position.clone(),
                                modal_mode: modal_mode.clone(),
                                show_project_add_or_edit_modal: show_project_add_or_edit_modal.clone(),
                                show_project_delete_modal: show_project_delete_modal.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}


// 项目新增或编辑组件
#[component]
pub fn ProjectAddOrEdit(
    mode: ProjectModalMode,
    on_confirm: Callback<(String, String, String)>,
    on_cancel: Callback<()>,
) -> Element {

    let mut project_code = use_signal(|| match &mode {
        ProjectModalMode::Add => String::new(),
        ProjectModalMode::Edit(project) => project.code.clone(),
    });
    let mut project_name = use_signal(|| match &mode {
        ProjectModalMode::Add => String::new(),
        ProjectModalMode::Edit(project) => project.name.clone(),
    });
    let mut project_description = use_signal(|| match &mode {
        ProjectModalMode::Add => String::new(),
        ProjectModalMode::Edit(project) => project.description.clone(),
    });

    let is_edit_mode = matches!(mode, ProjectModalMode::Edit(_));
    let modal_title = if is_edit_mode {
        "编辑项目"
    } else {
        "添加新项目"
    };

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

     // 校验：仅支持英文、数字、下划线，且首字符必须为英文
     let mut code_error = use_signal(|| None::<String>);
     let validate_code = crate::utils::validate::validate_code;

    rsx! {
        dialog {
            class: "modal modal-open",
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
                                oninput: move |event| {
                                    let value = event.value().to_string();
                                    let err = validate_code(&value);
                                    code_error.set(err);
                                    project_code.set(value);
                                }
                            }
                            if let Some(err) = code_error() {
                                span { class: "text-error text-xs mt-1", "{err}" }
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

// 项目删除组件
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
