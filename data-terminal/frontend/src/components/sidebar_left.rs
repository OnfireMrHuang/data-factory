use dioxus::prelude::*;
use dioxus::hooks::use_signal;
use dioxus_free_icons::{icons::{hi_outline_icons::*, fa_solid_icons::*, 
    md_content_icons::*, md_notification_icons::*, ld_icons::*}, Icon};
use crate::routes::Route;
use web_sys::window;

#[derive(PartialEq, Eq, Clone, Copy)]
enum MainMenu {
    DataCollection,
    DataProcessing,
    DataSupply,
    OpsMonitoring,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum SubMenu {
    // 数据采集
    DataSourceManagement,
    CollectionTasks,
    // 数据加工
    WideTableDevelopment,
    FileDevelopment,
    StreamDevelopment,
    // 数据供应
    DataQuery,
    DataSync,
    DataSubscription,
    // 运维监控
    CollectionTaskMonitoring,
    WideTableTaskMonitoring,
    FileTaskMonitoring,
    StreamTaskMonitoring,
}

// 持久化展开状态到本地存储
fn save_expanded_to_storage(expanded: Option<MainMenu>) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        let value = match expanded {
            Some(MainMenu::DataCollection) => "DataCollection",
            Some(MainMenu::DataProcessing) => "DataProcessing",
            Some(MainMenu::DataSupply) => "DataSupply",
            Some(MainMenu::OpsMonitoring) => "OpsMonitoring",
            None => "",
        };
        let _ = storage.set_item("sidebar_expanded", value);
    }
}

fn load_expanded_from_storage() -> Option<MainMenu> {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        if let Ok(Some(v)) = storage.get_item("sidebar_expanded") {
            return match v.as_str() {
                "DataCollection" => Some(MainMenu::DataCollection),
                "DataProcessing" => Some(MainMenu::DataProcessing),
                "DataSupply" => Some(MainMenu::DataSupply),
                "OpsMonitoring" => Some(MainMenu::OpsMonitoring),
                _ => None,
            };
        }
    }
    None
}

#[component]
pub fn SidebarLeft() -> Element {
    let mut expanded = use_signal(|| None as Option<MainMenu>);
    let mut selected = use_signal(|| None as Option<SubMenu>);
    let navigator = use_navigator();

    // 首次渲染时恢复展开状态
    let mut did_init = use_signal(|| false);
    if !did_init() {
        if let Some(init) = load_expanded_from_storage() {
            expanded.set(Some(init));
        }
        did_init.set(true);
    }

    // 计算所属主菜单的逻辑已不需要显式闭包（主菜单高亮只在无子菜单选中时生效）

    let render_submenu = |main: MainMenu| match main {
        MainMenu::DataCollection => {
            let source_class = {
                let mut class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                if selected() == Some(SubMenu::DataSourceManagement) {
                    class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                }
                class
            };
            let tasks_class = {
                let mut class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                if selected() == Some(SubMenu::CollectionTasks) {
                    class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                }
                class
            };
            rsx! {
                div { class: "pl-6 flex flex-col gap-2",
                    button {
                        class: source_class,
                        onclick: move |_| {
                            selected.set(Some(SubMenu::DataSourceManagement));
                            navigator.push(Route::DatasourceOverViewPage {});
                        },
                        Icon { icon: HiDatabase, class: "w-4 h-4" }
                        "数据源管理"
                    }
                    button {
                        class: tasks_class,
                        onclick: move |_| selected.set(Some(SubMenu::CollectionTasks)),
                        Icon { icon: FaDownload, class: "w-4 h-4" }
                        "采集任务"
                    }
                }
            }
        },
        MainMenu::DataProcessing => {
            let wide_class = {
                let mut class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                if selected() == Some(SubMenu::WideTableDevelopment) {
                    class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                }
                class
            };
            let file_class = {
                let mut class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                if selected() == Some(SubMenu::FileDevelopment) {
                    class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                }
                class
            };
            let stream_class = {
                let mut class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                if selected() == Some(SubMenu::StreamDevelopment) {
                    class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                }
                class
            };
            rsx! {
                div { class: "pl-6 flex flex-col gap-2",
                    button {
                        class: wide_class,
                        onclick: move |_| selected.set(Some(SubMenu::WideTableDevelopment)),
                        Icon { icon: HiTable, class: "w-4 h-4" }
                        "宽表开发"
                    }
                    button {
                        class: file_class,
                        onclick: move |_| selected.set(Some(SubMenu::FileDevelopment)),
                        Icon { icon: FaFile, class: "w-4 h-4" }
                        "文件开发"
                    }
                    button {
                        class: stream_class,
                        onclick: move |_| selected.set(Some(SubMenu::StreamDevelopment)),
                        Icon { icon: MdStream, class: "w-4 h-4" }
                        "流开发"
                    }
                }
            }
        },
        MainMenu::DataSupply => {
            let query_class = {
                let mut class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                if selected() == Some(SubMenu::DataQuery) {
                    class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                }
                class
            };
            let sync_class = {
                let mut class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                if selected() == Some(SubMenu::DataSync) {
                    class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                }
                class
            };
            let subscription_class = {
                let mut class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                if selected() == Some(SubMenu::DataSubscription) {
                    class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                }
                class
            };
            rsx! {
                div { class: "pl-6 flex flex-col gap-2",
                    button {
                        class: query_class,
                        onclick: move |_| selected.set(Some(SubMenu::DataQuery)),
                        Icon { icon: HiSearch, class: "w-4 h-4" }
                        "数据查询"
                    }
                    button {
                        class: sync_class,
                        onclick: move |_| selected.set(Some(SubMenu::DataSync)),
                        Icon { icon: MdSync, class: "w-4 h-4" }
                        "数据同步"
                    }
                    button {
                        class: subscription_class,
                        onclick: move |_| selected.set(Some(SubMenu::DataSubscription)),
                        Icon { icon: HiBell, class: "w-4 h-4" }
                        "数据订阅"
                    }
                }
            }
        },
        MainMenu::OpsMonitoring => {
            let collection_class = {
                let mut class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                if selected() == Some(SubMenu::CollectionTaskMonitoring) {
                    class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                }
                class
            };
            let wide_class = {
                let mut class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                if selected() == Some(SubMenu::WideTableTaskMonitoring) {
                    class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                }
                class
            };
            let file_class = {
                let mut class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                if selected() == Some(SubMenu::FileTaskMonitoring) {
                    class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                }
                class
            };
            let stream_class = {
                let mut class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                if selected() == Some(SubMenu::StreamTaskMonitoring) {
                    class = "btn btn-ghost text-sm font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                }
                class
            };
            rsx! {
                div { class: "pl-6 flex flex-col gap-2",
                    button {
                        class: collection_class,
                        onclick: move |_| selected.set(Some(SubMenu::CollectionTaskMonitoring)),
                        Icon { icon: HiEye, class: "w-4 h-4" }
                        "采集任务监控"
                    }
                    button {
                        class: wide_class,
                        onclick: move |_| selected.set(Some(SubMenu::WideTableTaskMonitoring)),
                        Icon { icon: HiTable, class: "w-4 h-4" }
                        "宽表任务监控"
                    }
                    button {
                        class: file_class,
                        onclick: move |_| selected.set(Some(SubMenu::FileTaskMonitoring)),
                        Icon { icon: FaFile, class: "w-4 h-4" }
                        "文件任务监控"
                    }
                    button {
                        class: stream_class,
                        onclick: move |_| selected.set(Some(SubMenu::StreamTaskMonitoring)),
                        Icon { icon: MdStream, class: "w-4 h-4" }
                        "流任务监控"
                    }
                }
            }
        },
    };

    rsx! {
        aside { 
            class: "w-56 bg-base-200 flex flex-col p-4",
            // 主页一级菜单按钮
            button {
                class: "btn btn-ghost text-base font-medium flex items-center gap-3 text-primary w-full justify-start mb-4",
                onclick: move |_| {
                    selected.set(None);
                    navigator.push(Route::Home {});
                },
                Icon { icon: HiHome, class: "w-5 h-5" }
                span { "主页" }
            }
            // 数据采集
            button {
                class: {
                    let mut class = "btn btn-ghost text-base font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                    let any_sub_selected = selected().is_some();
                    if expanded() == Some(MainMenu::DataCollection) && !any_sub_selected {
                        class = "btn btn-ghost text-base font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                    }
                    class
                },
                onclick: move |_| {
                    let new_state = if expanded() == Some(MainMenu::DataCollection) { None } else { Some(MainMenu::DataCollection) };
                    expanded.set(new_state);
                    selected.set(None);
                    save_expanded_to_storage(new_state);
                },
                Icon { icon: HiCollection, class: "w-5 h-5" }
                span { "数据采集" }
                span {
                    class: "text-base-content/50 ml-auto",
                    { if expanded() == Some(MainMenu::DataCollection) {
                        rsx!(Icon { icon: HiChevronDown, class: "w-4 h-4" })
                    } else {
                        rsx!(Icon { icon: HiChevronRight, class: "w-4 h-4" })
                    }}
                }
            }
            { if expanded() == Some(MainMenu::DataCollection) { render_submenu(MainMenu::DataCollection) } else { rsx!{} } }

            // 数据加工
            button {
                class: {
                    let mut class = "btn btn-ghost text-base font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                    let any_sub_selected = selected().is_some();
                    if expanded() == Some(MainMenu::DataProcessing) && !any_sub_selected {
                        class = "btn btn-ghost text-base font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                    }
                    class
                },
                onclick: move |_| {
                    let new_state = if expanded() == Some(MainMenu::DataProcessing) { None } else { Some(MainMenu::DataProcessing) };
                    expanded.set(new_state);
                    selected.set(None);
                    save_expanded_to_storage(new_state);
                },
                Icon { icon: FaHammer, class: "w-5 h-5" }
                span { "数据加工" }
                span {
                    class: "text-base-content/50 ml-auto",
                    { if expanded() == Some(MainMenu::DataProcessing) {
                        rsx!(Icon { icon: HiChevronDown, class: "w-4 h-4" })
                    } else {
                        rsx!(Icon { icon: HiChevronRight, class: "w-4 h-4" })
                    }}
                }
            }
            { if expanded() == Some(MainMenu::DataProcessing) { render_submenu(MainMenu::DataProcessing) } else { rsx!{} } }

            // 数据供应
            button {
                class: {
                    let mut class = "btn btn-ghost text-base font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                    let any_sub_selected = selected().is_some();
                    if expanded() == Some(MainMenu::DataSupply) && !any_sub_selected {
                        class = "btn btn-ghost text-base font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                    }
                    class
                },
                onclick: move |_| {
                    let new_state = if expanded() == Some(MainMenu::DataSupply) { None } else { Some(MainMenu::DataSupply) };
                    expanded.set(new_state);
                    selected.set(None);
                    save_expanded_to_storage(new_state);
                },
                Icon { icon: HiShare, class: "w-5 h-5" }
                span { "数据供应" }
                span {
                    class: "text-base-content/50 ml-auto",
                    { if expanded() == Some(MainMenu::DataSupply) {
                        rsx!(Icon { icon: HiChevronDown, class: "w-4 h-4" })
                    } else {
                        rsx!(Icon { icon: HiChevronRight, class: "w-4 h-4" })
                    }}
                }
            }
            { if expanded() == Some(MainMenu::DataSupply) { render_submenu(MainMenu::DataSupply) } else { rsx!{} } }

            // 运维监控
            button {
                class: {
                    let mut class = "btn btn-ghost text-base font-medium flex items-center gap-3 w-full justify-start text-base-content/70 hover:text-base-content".to_string();
                    let any_sub_selected = selected().is_some();
                    if expanded() == Some(MainMenu::OpsMonitoring) && !any_sub_selected {
                        class = "btn btn-ghost text-base font-medium flex items-center gap-3 w-full justify-start text-primary bg-primary/10".to_string();
                    }
                    class
                },
                onclick: move |_| {
                    let new_state = if expanded() == Some(MainMenu::OpsMonitoring) { None } else { Some(MainMenu::OpsMonitoring) };
                    expanded.set(new_state);
                    selected.set(None);
                    save_expanded_to_storage(new_state);
                },
                Icon { icon: LdMonitorCheck, class: "w-5 h-5" }
                span { "运维监控" }
                span {
                    class: "text-base-content/50 ml-auto",
                    { if expanded() == Some(MainMenu::OpsMonitoring) {
                        rsx!(Icon { icon: HiChevronDown, class: "w-4 h-4" })
                    } else {
                        rsx!(Icon { icon: HiChevronRight, class: "w-4 h-4" })
                    }}
                }
            }
            { if expanded() == Some(MainMenu::OpsMonitoring) { render_submenu(MainMenu::OpsMonitoring) } else { rsx!{} } }
        }
    }
}
