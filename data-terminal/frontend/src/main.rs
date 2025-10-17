pub mod models;
pub mod components;
pub mod pages;
pub mod routes;
pub mod utils;

use dioxus::prelude::*;
use crate::routes::AppRouter;
use dioxus::logger::tracing::{Level};


fn main() {
    // Initialize logger with INFO level for web console output
    dioxus::logger::init(Level::INFO).expect("logger failed to init");

    tracing::info!("Application starting...");

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