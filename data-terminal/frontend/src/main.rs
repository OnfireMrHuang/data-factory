pub mod models;
pub mod components;
pub mod pages;
pub mod routes;
pub mod utils;

use dioxus::prelude::*;
use crate::routes::AppRouter;
use dioxus::logger::tracing::{Level};


fn main() {
    dioxus::logger::init(Level::INFO).expect("logger failed to init");

    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        document::Stylesheet {
            // Urls are relative to your Cargo.toml file
            href: asset!("/assets/tailwind.css")
        }
        AppRouter {}
    }
}