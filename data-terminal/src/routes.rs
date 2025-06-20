use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::pages::{home::Home, login::Login};

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    #[route("/home")]
    Home {},

    #[route("/login")]
    Login {},
}

pub fn AppRouter() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
