use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::{navbar::Navbar, sidebar_left::SidebarLeft, sidebar_right::SidebarRight};
use crate::{pages::{catch_all::PageNotFound, home::Home, login::Login, resource::ResourcePage, datasource_overview::DatasourceOverViewPage, datasource_mysql_config::DatasourceMysqlConfig}};

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
            #[route("/mysql/config")]
            DatasourceMysqlConfig {},
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
                Outlet::<Route> {}
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
