use dioxus::prelude::*;


#[component]
pub fn Content() -> Element {
    rsx! {
        main { class: "flex-1 bg-white p-8", /* 中间内容，默认留空 */ }
    }
}