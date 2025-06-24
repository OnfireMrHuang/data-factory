mod components;
mod pages;
mod routes;

use dioxus::prelude::*;
use crate::routes::AppRouter;


fn main() {
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