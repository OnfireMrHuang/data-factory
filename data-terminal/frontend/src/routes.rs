use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{pages::{catch_all::PageNotFound, home::Home, login::Login}};

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {

    #[route("/login")]
    Login {},

    #[route("/")]
    Home {},


    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

pub fn AppRouter() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
