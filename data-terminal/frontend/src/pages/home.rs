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
                    
                    // Cookie调试信息
                    div { class: "bg-base-200 p-4 rounded-lg",
                        h2 { class: "text-xl font-semibold mb-2", "Cookie调试信息" }
                        p { class: "text-sm text-gray-600 mb-2", "当前所有Cookie:" }
                        pre { class: "bg-base-300 p-2 rounded text-xs overflow-x-auto", "{cookies}" }
                    }
                }
            })
        }
    }
} 