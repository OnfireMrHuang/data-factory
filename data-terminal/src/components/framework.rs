use dioxus::prelude::*;
use crate::components::{navbar::Navbar, sidebar_left::SidebarLeft, sidebar_right::SidebarRight, content::Content};

#[component]
pub fn Framework() -> Element {
    rsx! {
        div { class: "flex flex-col h-screen",
            Navbar {}
            div { class: "flex flex-1",
                SidebarLeft {}
                Content {}
                SidebarRight {}
            }
        }
    }
}
