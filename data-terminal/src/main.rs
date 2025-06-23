mod components;
mod pages;
mod routes;

use dioxus::prelude::*;
use crate::routes::AppRouter;
use axum::{response::Html, routing::get, Router};


#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("listening on http://127.0.0.1:3000");

    axum::serve(
        listener,
        Router::new()
            .route("/", get(app_endpoint))
            .into_make_service(),
    )
    .await
    .unwrap();
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

async fn app_endpoint() -> Html<String> {
    // create a VirtualDom with the app component
    let mut app = VirtualDom::new(app);
    // rebuild the VirtualDom before rendering
    app.rebuild_in_place();

    // render the VirtualDom to HTML
    Html(dioxus_ssr::render(&app))
}