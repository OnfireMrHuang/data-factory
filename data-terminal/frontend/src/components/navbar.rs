use dioxus::prelude::*;
use super::project_selector::ProjectSelector;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "flex items-center justify-between h-16 px-4 bg-base-300",
            // 左侧项目选择器
            ProjectSelector {}
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
                button { class: "btn flex items-center gap-1",
                    // 齿轮图标
                    svg { width: "18", height: "18", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M12 15.5A3.5 3.5 0 1 0 12 8.5a3.5 3.5 0 0 0 0 7z", stroke_linecap: "round", stroke_linejoin: "round" }
                        path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09a1.65 1.65 0 0 0-1-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09c.7 0 1.31-.4 1.51-1a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33h.09c.7 0 1.31-.4 1.51-1V3a2 2 0 1 1 4 0v.09c0 .7.4 1.31 1 1.51a1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82v.09c0 .7.4 1.31 1 1.51a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09c-.7 0-1.31.4-1.51 1z", stroke_linecap: "round", stroke_linejoin: "round" }
                    }
                    "设置"
                }
            }
        }
    }
}