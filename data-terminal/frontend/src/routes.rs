use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::{navbar::Navbar, sidebar_left::SidebarLeft, sidebar_right::SidebarRight};
use crate::{pages::{
    catch_all::PageNotFound,
    home::Home,
    login::Login,
    resource::ResourcePage,
    datasource_overview::DatasourceOverViewPage,
    datasource_mysql_config::{DatasourceMysqlAdd, DatasourceMysqlEdit},
    datasource_postgres_config::{DatasourcePostgresAdd, DatasourcePostgresEdit},
    datasource_queryapi_config::{DatasourceQueryApiAdd, DatasourceQueryApiEdit},
    datasource_subscribeapi_config::{DatasourceSubscribeApiAdd, DatasourceSubscribeApiEdit, DatasourceSubscribeApiTokenManagement},
    collection_page::CollectionPage,
    collection_create_page::CollectionCreatePage,
    collection_edit_page::CollectionEditPage,
}};

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
            #[nest("/postgres")]
                #[route("/add")]
                DatasourcePostgresAdd {},
                #[route("/edit/:id")]
                DatasourcePostgresEdit { id: String },
            #[end_nest]
            #[nest("/queryapi")]
                #[route("/add")]
                DatasourceQueryApiAdd {},
                #[route("/edit/:id")]
                DatasourceQueryApiEdit { id: String },
            #[end_nest]
            #[nest("/subscribeapi")]
                #[route("/add")]
                DatasourceSubscribeApiAdd {},
                #[route("/edit/:id")]
                DatasourceSubscribeApiEdit { id: String },
                #[route("/tokens/:id")]
                DatasourceSubscribeApiTokenManagement { id: String },
            #[end_nest]
        #[end_nest]

        #[nest("/collection")]
            #[route("/")]
            CollectionPage {},
            #[route("/create")]
            CollectionCreatePage {},
            #[route("/edit/:id")]
            CollectionEditPage { id: String },
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
