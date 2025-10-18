use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::{navbar::Navbar, sidebar_left::SidebarLeft, sidebar_right::SidebarRight};
use crate::{pages::{catch_all::PageNotFound, home::Home, login::Login, resource::ResourcePage, datasource_overview::DatasourceOverViewPage, datasource_mysql_config::{DatasourceMysqlAdd, DatasourceMysqlEdit}}};

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {

    #[route("/login")]
    Login {},

    #[layout(Framework)]
        #[route("/")]
        Home {},

        #[nest("/datasource")]
            #[route("/")]
            DatasourceOverViewPage {},
            #[nest("/mysql")]
                #[route("/add")]
                DatasourceMysqlAdd {},
                #[route("/edit/:id")]
                DatasourceMysqlEdit { id: String },
            #[end_nest]
        #[end_nest]
    #[end_layout]

    #[route("/resource")]
    ResourcePage {},

    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
pub fn Framework() -> Element {
    rsx! {
        div { class: "flex flex-col h-screen",
            Navbar {}
            div { class: "flex flex-1",
                SidebarLeft {}
                div { class: "flex-1 overflow-auto",
                    Outlet::<Route> {}
                }
                SidebarRight {}
            }
        }
    }
}

pub fn AppRouter() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
