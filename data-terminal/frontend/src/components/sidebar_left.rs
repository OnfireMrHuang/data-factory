use dioxus::prelude::*;
use dioxus::hooks::use_signal;

#[derive(PartialEq, Eq, Clone, Copy)]
enum MainMenu {
    DataIntegration,
    DataDevelopment,
    VersionManagement,
    OpsMonitoring,
    DataSupply,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum SubMenu {
    DatabaseIntegration,
    CustomIntegration,
    WideTableDevelopment,
    CustomDevelopment,
    VersionList,
    Archive,
    Rollback,
    WideTableTasks,
    StreamTasks,
    BatchTasks,
    DataService,
    DataSync,
}

#[component]
pub fn SidebarLeft() -> Element {
    let mut expanded = use_signal(|| None as Option<MainMenu>);
    let mut selected = use_signal(|| None as Option<SubMenu>);

    let submenu_of = |submenu: SubMenu| match submenu {
        SubMenu::DatabaseIntegration | SubMenu::CustomIntegration => Some(MainMenu::DataIntegration),
        SubMenu::WideTableDevelopment | SubMenu::CustomDevelopment => Some(MainMenu::DataDevelopment),
        SubMenu::VersionList | SubMenu::Archive | SubMenu::Rollback => Some(MainMenu::VersionManagement),
        SubMenu::WideTableTasks | SubMenu::StreamTasks | SubMenu::BatchTasks => Some(MainMenu::OpsMonitoring),
        SubMenu::DataService | SubMenu::DataSync => Some(MainMenu::DataSupply),
    };

    let render_submenu = |main: MainMenu| match main {
        MainMenu::DataIntegration => {
            let db_class = {
                let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                if selected() == Some(SubMenu::DatabaseIntegration) {
                    class.push_str(" text-sky-500");
                }
                class
            };
            let custom_class = {
                let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                if selected() == Some(SubMenu::CustomIntegration) {
                    class.push_str(" text-sky-500");
                }
                class
            };
            rsx! {
                div { class: "pl-4 flex flex-col gap-1",
                    button {
                        class: db_class,
                        onclick: move |_| selected.set(Some(SubMenu::DatabaseIntegration)),
                        "数据库集成"
                    }
                    button {
                        class: custom_class,
                        onclick: move |_| selected.set(Some(SubMenu::CustomIntegration)),
                        "自定义集成"
                    }
                }
            }
        },
        MainMenu::DataDevelopment => {
            let wide_class = {
                let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                if selected() == Some(SubMenu::WideTableDevelopment) {
                    class.push_str(" text-sky-500");
                }
                class
            };
            let custom_class = {
                let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                if selected() == Some(SubMenu::CustomDevelopment) {
                    class.push_str(" text-sky-500");
                }
                class
            };
            rsx! {
                div { class: "pl-4 flex flex-col gap-1",
                    button {
                        class: wide_class,
                        onclick: move |_| selected.set(Some(SubMenu::WideTableDevelopment)),
                        "宽表开发"
                    }
                    button {
                        class: custom_class,
                        onclick: move |_| selected.set(Some(SubMenu::CustomDevelopment)),
                        "自定义开发"
                    }
                }
            }
        },
        MainMenu::VersionManagement => {
            let list_class = {
                let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                if selected() == Some(SubMenu::VersionList) {
                    class.push_str(" text-sky-500");
                }
                class
            };
            let archive_class = {
                let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                if selected() == Some(SubMenu::Archive) {
                    class.push_str(" text-sky-500");
                }
                class
            };
            let rollback_class = {
                let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                if selected() == Some(SubMenu::Rollback) {
                    class.push_str(" text-sky-500");
                }
                class
            };
            rsx! {
                div { class: "pl-4 flex flex-col gap-1",
                    button {
                        class: list_class,
                        onclick: move |_| selected.set(Some(SubMenu::VersionList)),
                        "版本列表"
                    }
                    button {
                        class: archive_class,
                        onclick: move |_| selected.set(Some(SubMenu::Archive)),
                        "归档"
                    }
                    button {
                        class: rollback_class,
                        onclick: move |_| selected.set(Some(SubMenu::Rollback)),
                        "回滚"
                    }
                }
            }
        },
        MainMenu::OpsMonitoring => {
            let wide_class = {
                let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                if selected() == Some(SubMenu::WideTableTasks) {
                    class.push_str(" text-sky-500");
                }
                class
            };
            let stream_class = {
                let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                if selected() == Some(SubMenu::StreamTasks) {
                    class.push_str(" text-sky-500");
                }
                class
            };
            let batch_class = {
                let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                if selected() == Some(SubMenu::BatchTasks) {
                    class.push_str(" text-sky-500");
                }
                class
            };
            rsx! {
                div { class: "pl-4 flex flex-col gap-1",
                    button {
                        class: wide_class,
                        onclick: move |_| selected.set(Some(SubMenu::WideTableTasks)),
                        "宽表任务列表"
                    }
                    button {
                        class: stream_class,
                        onclick: move |_| selected.set(Some(SubMenu::StreamTasks)),
                        "流任务列表"
                    }
                    button {
                        class: batch_class,
                        onclick: move |_| selected.set(Some(SubMenu::BatchTasks)),
                        "批任务列表"
                    }
                }
            }
        },
        MainMenu::DataSupply => {
            let service_class = {
                let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                if selected() == Some(SubMenu::DataService) {
                    class.push_str(" text-sky-500");
                }
                class
            };
            let sync_class = {
                let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                if selected() == Some(SubMenu::DataSync) {
                    class.push_str(" text-sky-500");
                }
                class
            };
            rsx! {
                div { class: "pl-4 flex flex-col gap-1",
                    button {
                        class: service_class,
                        onclick: move |_| selected.set(Some(SubMenu::DataService)),
                        "数据服务"
                    }
                    button {
                        class: sync_class,
                        onclick: move |_| selected.set(Some(SubMenu::DataSync)),
                        "数据同步"
                    }
                }
            }
        },
    };

    rsx! {
        aside { 
            class: "w-48 bg-base-200 flex flex-col p-4",
            // 主页一级菜单按钮
            button {
                class: "btn btn-ghost btn-sm flex items-center gap-2 text-sky-500 w-full justify-start",
                // SVG 占位符 icon
                svg { width: "16", height: "16", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M3 12l9-9 9 9M4 10v10a1 1 0 0 0 1 1h5a1 1 0 0 0 1-1v-4a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v4a1 1 0 0 0 1 1h5a1 1 0 0 0 1-1V10" }
                }
                span { "主页" }
            }
            // 一级菜单
            button {
                class: {
                    let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                    let is_selected = selected().map_or(false, |s| submenu_of(s) == Some(MainMenu::DataIntegration));
                    if is_selected || expanded() == Some(MainMenu::DataIntegration) {
                        class.push_str(" text-sky-500");
                    }
                    class
                },
                onclick: move |_| expanded.set(
                    if expanded() == Some(MainMenu::DataIntegration) { None } else { Some(MainMenu::DataIntegration) }
                ),
                // SVG 占位符 icon
                svg { width: "16", height: "16", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M4 12h16M12 4v16", stroke_linecap: "round", stroke_linejoin: "round" }
                }
                span { "数据集成" }
                span {
                    class: "text-gray-400 text-xs ml-auto",
                    { if expanded() == Some(MainMenu::DataIntegration) {
                        rsx!(svg { width: "12", height: "12", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg", path { d: "M6 9l6 6 6-6", stroke_linecap: "round", stroke_linejoin: "round" } })
                    } else {
                        rsx!(svg { width: "12", height: "12", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg", path { d: "M9 6l6 6-6 6", stroke_linecap: "round", stroke_linejoin: "round" } })
                    }}
                }
            }
            { if expanded() == Some(MainMenu::DataIntegration) { render_submenu(MainMenu::DataIntegration) } else { rsx!{} } }

            button {
                class: {
                    let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                    let is_selected = selected().map_or(false, |s| submenu_of(s) == Some(MainMenu::DataDevelopment));
                    if is_selected || expanded() == Some(MainMenu::DataDevelopment) {
                        class.push_str(" text-sky-500");
                    }
                    class
                },
                onclick: move |_| expanded.set(
                    if expanded() == Some(MainMenu::DataDevelopment) { None } else { Some(MainMenu::DataDevelopment) }
                ),
                svg { width: "16", height: "16", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                    circle { cx: "12", cy: "12", r: "8", stroke_linecap: "round", stroke_linejoin: "round" }
                }
                span { "数据开发" }
                span {
                    class: "text-gray-400 text-xs ml-auto",
                    { if expanded() == Some(MainMenu::DataDevelopment) {
                        rsx!(svg { width: "12", height: "12", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg", path { d: "M6 9l6 6 6-6", stroke_linecap: "round", stroke_linejoin: "round" } })
                    } else {
                        rsx!(svg { width: "12", height: "12", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg", path { d: "M9 6l6 6-6 6", stroke_linecap: "round", stroke_linejoin: "round" } })
                    }}
                }
            }
            { if expanded() == Some(MainMenu::DataDevelopment) { render_submenu(MainMenu::DataDevelopment) } else { rsx!{} } }

            button {
                class: {
                    let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                    let is_selected = selected().map_or(false, |s| submenu_of(s) == Some(MainMenu::VersionManagement));
                    if is_selected || expanded() == Some(MainMenu::VersionManagement) {
                        class.push_str(" text-sky-500");
                    }
                    class
                },
                onclick: move |_| expanded.set(
                    if expanded() == Some(MainMenu::VersionManagement) { None } else { Some(MainMenu::VersionManagement) }
                ),
                svg { width: "16", height: "16", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                    rect { x: "4", y: "4", width: "16", height: "16", rx: "2", stroke_linecap: "round", stroke_linejoin: "round" }
                }
                span { "版本管理" }
                span {
                    class: "text-gray-400 text-xs ml-auto",
                    { if expanded() == Some(MainMenu::VersionManagement) {
                        rsx!(svg { width: "12", height: "12", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg", path { d: "M6 9l6 6 6-6", stroke_linecap: "round", stroke_linejoin: "round" } })
                    } else {
                        rsx!(svg { width: "12", height: "12", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg", path { d: "M9 6l6 6-6 6", stroke_linecap: "round", stroke_linejoin: "round" } })
                    }}
                }
            }
            { if expanded() == Some(MainMenu::VersionManagement) { render_submenu(MainMenu::VersionManagement) } else { rsx!{} } }

            button {
                class: {
                    let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                    let is_selected = selected().map_or(false, |s| submenu_of(s) == Some(MainMenu::OpsMonitoring));
                    if is_selected || expanded() == Some(MainMenu::OpsMonitoring) {
                        class.push_str(" text-sky-500");
                    }
                    class
                },
                onclick: move |_| expanded.set(
                    if expanded() == Some(MainMenu::OpsMonitoring) { None } else { Some(MainMenu::OpsMonitoring) }
                ),
                svg { width: "16", height: "16", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                    polyline { points: "4 12 12 4 20 12", stroke_linecap: "round", stroke_linejoin: "round" }
                }
                span { "运维监控" }
                span {
                    class: "text-gray-400 text-xs ml-auto",
                    { if expanded() == Some(MainMenu::OpsMonitoring) {
                        rsx!(svg { width: "12", height: "12", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg", path { d: "M6 9l6 6 6-6", stroke_linecap: "round", stroke_linejoin: "round" } })
                    } else {
                        rsx!(svg { width: "12", height: "12", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg", path { d: "M9 6l6 6-6 6", stroke_linecap: "round", stroke_linejoin: "round" } })
                    }}
                }
            }
            { if expanded() == Some(MainMenu::OpsMonitoring) { render_submenu(MainMenu::OpsMonitoring) } else { rsx!{} } }

            button {
                class: {
                    let mut class = "btn btn-ghost btn-sm flex items-center gap-2 w-full".to_string();
                    let is_selected = selected().map_or(false, |s| submenu_of(s) == Some(MainMenu::DataSupply));
                    if is_selected || expanded() == Some(MainMenu::DataSupply) {
                        class.push_str(" text-sky-500");
                    }
                    class
                },
                onclick: move |_| expanded.set(
                    if expanded() == Some(MainMenu::DataSupply) { None } else { Some(MainMenu::DataSupply) }
                ),
                svg { width: "16", height: "16", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M12 4v16m8-8H4", stroke_linecap: "round", stroke_linejoin: "round" }
                }
                span { "数据供应" }
                span {
                    class: "text-gray-400 text-xs ml-auto",
                    { if expanded() == Some(MainMenu::DataSupply) {
                        rsx!(svg { width: "12", height: "12", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg", path { d: "M6 9l6 6 6-6", stroke_linecap: "round", stroke_linejoin: "round" } })
                    } else {
                        rsx!(svg { width: "12", height: "12", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg", path { d: "M9 6l6 6-6 6", stroke_linecap: "round", stroke_linejoin: "round" } })
                    }}
                }
            }
            { if expanded() == Some(MainMenu::DataSupply) { render_submenu(MainMenu::DataSupply) } else { rsx!{} } }
        }
    }
}