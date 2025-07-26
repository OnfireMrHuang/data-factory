use dioxus::prelude::*;
use crate::components::framework::Framework;
use crate::utils::cookie;

#[component]
pub fn Home() -> Element {
    let cookies = use_signal(|| cookie::get_browser_cookies());
    
    rsx! {
        Framework {
            children: Some(rsx! {
                div { class: "p-6",
                    h1 { class: "text-3xl font-bold mb-4", "欢迎使用数据工厂" }
                    p { class: "text-lg text-gray-700 mb-6", "数据工厂可向您提供数据集成、开发和服务，同时支持配置您的数据流；方便个人专注于数据处理和分析，免于处理复杂的环境问题。" }
                }
            })
        }
    }
} 