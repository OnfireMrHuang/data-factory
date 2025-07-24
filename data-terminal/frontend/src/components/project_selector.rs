use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::utils::{cookie};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub code: String,
    pub name: String,
    pub description: String,
    pub logo: String,
    pub create_status: String,
    pub create_msg: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub result: bool,
    pub msg: String,
    pub data: T,
}

#[component]
pub fn ProjectSelector() -> Element {
    let mut projects = use_signal(|| Vec::<Project>::new());
    let mut loading = use_signal(|| false);
    let mut show_dropdown = use_signal(|| false);
    let mut selected_project = use_signal(|| None as Option<Project>);

    // 获取项目列表
    let fetch_projects = {
        let mut projects = projects.clone();
        let mut loading = loading.clone();
        move || {
            loading.set(true);
            spawn(async move {
                let cookie = cookie::get_browser_cookies();
                tracing::info!("cookie: {:?}", cookie);
                let client = reqwest::Client::new();
                let response = client
                    .get("http://127.0.0.1:3000/api/v1/project/list")
                    .header("Cookie", cookie)
                    .send()
                    .await;
                
                match response {
                    Ok(resp) => {
                        if let Ok(api_response) = resp.json::<ApiResponse<Vec<Project>>>().await {
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

    rsx! {
        div { class: "relative",
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
                                onclick: move |_| {
                                    // TODO: 打开新增项目对话框
                                    log::info!("Open new project dialog");
                                },
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
                                div { class: "p-4 text-center text-base-content/60",
                                    "存在项目"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
} 