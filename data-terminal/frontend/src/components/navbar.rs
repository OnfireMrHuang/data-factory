use dioxus::prelude::*;
use dioxus_router::prelude::*;
use super::project_panel::ProjectPanel;
use crate::routes::Route;

// 定义菜单项结构
#[derive(Clone, PartialEq)]
struct MenuItem {
    id: &'static str,
    label: &'static str,
    icon: &'static str,
    route: Option<Route>,
}

// 设置菜单项列表 - 便于后续扩展
fn get_settings_menu_items() -> Vec<MenuItem> {
    vec![
        MenuItem {
            id: "resource_settings",
            label: "资源设置",
            icon: "database",
            route: Some(Route::ResourcePage {}),
        },
        // 可以在这里添加更多菜单项
        // MenuItem {
        //     id: "user_settings",
        //     label: "用户设置",
        //     icon: "user",
        //     route: Some(Route::UserSettings {}),
        // },
        // MenuItem {
        //     id: "system_settings",
        //     label: "系统设置",
        //     icon: "cog",
        //     route: Some(Route::SystemSettings {}),
        // },
    ]
}

#[component]
pub fn Navbar() -> Element {
    let navigator = use_navigator();
    let mut is_dropdown_open = use_signal(|| false);
    
    // 获取菜单项
    let menu_items = use_signal(|| get_settings_menu_items());
    
    // 处理菜单项点击
    let mut handle_menu_click = move |route: Route| {
        navigator.push(route);
        is_dropdown_open.set(false);
    };
    
    // 处理设置按钮点击
    let mut handle_settings_click = move |_| {
        let current = *is_dropdown_open.read();
        is_dropdown_open.set(!current);
    };
    
    // 处理点击外部关闭下拉菜单
    let handle_click_outside = move |_| {
        is_dropdown_open.set(false);
    };

    rsx! {
        nav { class: "flex items-center justify-between h-16 px-4 bg-base-300",
            // 左侧项目选择器
            ProjectPanel {}
            // 中间 input
            div { class: "flex-1 flex justify-start",
                div { class: "relative w-1/5 mx-4",
                    svg { class: "absolute left-2 top-1/2 -translate-y-1/2 text-gray-400", width: "18", height: "18", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                        circle { cx: "11", cy: "11", r: "8" }
                        line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                    }
                    input {
                        class: "w-full pl-8 pr-2 py-1 bg-base-200 border-none shadow-none text-base-content rounded focus:outline-none focus:ring-0",
                        r#type: "text",
                        placeholder: "搜索..."
                    }
                }
            }
            // 右侧按钮
            div { class: "flex gap-2 items-center",
                button { class: "btn flex items-center gap-1",
                    // AI 脑图标
                    svg { width: "18", height: "18", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M12 2C7 2 2 7 2 12s5 10 10 10 10-5 10-10S17 2 12 2z", stroke_linecap: "round", stroke_linejoin: "round" }
                        path { d: "M8 15s1.5-2 4-2 4 2 4 2", stroke_linecap: "round", stroke_linejoin: "round" }
                        circle { cx: "9", cy: "10", r: "1" }
                        circle { cx: "15", cy: "10", r: "1" }
                    }
                    "智能助手"
                }
                button { class: "btn flex items-center gap-1",
                    // 锤子图标
                    svg { width: "18", height: "18", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M17 7l-1.5-1.5a2.121 2.121 0 0 0-3 0l-7.5 7.5a2.121 2.121 0 0 0 0 3l1.5 1.5a2.121 2.121 0 0 0 3 0l7.5-7.5a2.121 2.121 0 0 0 0-3z", stroke_linecap: "round", stroke_linejoin: "round" }
                    }
                    "工具"
                }
                // 设置按钮和下拉菜单
                div { class: "relative",
                    button { 
                        class: "btn flex items-center gap-1",
                        onclick: handle_settings_click,
                        // 齿轮图标
                        svg { width: "18", height: "18", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M12 15.5A3.5 3.5 0 1 0 12 8.5a3.5 3.5 0 0 0 0 7z", stroke_linecap: "round", stroke_linejoin: "round" }
                            path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09a1.65 1.65 0 0 0-1-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09c.7 0 1.31-.4 1.51-1a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33h.09c.7 0 1.31-.4 1.51-1V3a2 2 0 1 1 4 0v.09c0 .7.4 1.31 1 1.51a1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82v.09c0 .7.4 1.31 1 1.51a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09c-.7 0-1.31.4-1.51 1z", stroke_linecap: "round", stroke_linejoin: "round" }
                        }
                        "设置"
                    }
                    
                    // 下拉菜单
                    if *is_dropdown_open.read() {
                        div { 
                            class: "absolute right-0 mt-2 w-48 bg-base-100 rounded-md shadow-lg z-50 border border-base-300",
                            onclick: move |e| e.stop_propagation(),
                            // 菜单项列表
                            ul { class: "py-1",
                                for item in menu_items() {
                                    li {
                                        button {
                                            class: "w-full text-left px-4 py-2 text-sm text-base-content hover:bg-base-200 flex items-center gap-2",
                                            onclick: move |_| {
                                                if let Some(route) = &item.route {
                                                    handle_menu_click(route.clone());
                                                }
                                            },
                                            // 根据图标类型显示不同的图标
                                            if item.icon == "database" {
                                                svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                                                    ellipse { cx: "12", cy: "5", rx: "9", ry: "3" }
                                                    path { d: "M3 5v14c0 1.66 4.03 3 9 3s9-1.34 9-3V5", stroke_linecap: "round", stroke_linejoin: "round" }
                                                    path { d: "M3 12c0 1.66 4.03 3 9 3s9-1.34 9-3", stroke_linecap: "round", stroke_linejoin: "round" }
                                                }
                                            } else if item.icon == "user" {
                                                svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                                                    path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2", stroke_linecap: "round", stroke_linejoin: "round" }
                                                    circle { cx: "12", cy: "7", r: "4" }
                                                }
                                            } else if item.icon == "cog" {
                                                svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                                                    path { d: "M12 15.5A3.5 3.5 0 1 0 12 8.5a3.5 3.5 0 0 0 0 7z", stroke_linecap: "round", stroke_linejoin: "round" }
                                                    path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09a1.65 1.65 0 0 0-1-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09c.7 0 1.31-.4 1.51-1a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33h.09c.7 0 1.31-.4 1.51-1V3a2 2 0 1 1 4 0v.09c0 .7.4 1.31 1 1.51a1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82v.09c0 .7.4 1.31 1 1.51a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09c-.7 0-1.31.4-1.51 1z", stroke_linecap: "round", stroke_linejoin: "round" }
                                                }
                                            }
                                            {item.label}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // 点击外部关闭下拉菜单的遮罩层
        if *is_dropdown_open.read() {
            div { 
                class: "fixed inset-0 z-40",
                onclick: handle_click_outside
            }
        }
    }
}