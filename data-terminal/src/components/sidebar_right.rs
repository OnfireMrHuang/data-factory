use dioxus::prelude::*;

#[component]
pub fn SidebarRight() -> Element {
    rsx! {
        aside { class: "w-64 bg-base-100 border-l hidden", /* 右侧辅助栏，默认留空 */ }
    }
}