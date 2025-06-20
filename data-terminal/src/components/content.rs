use dioxus::prelude::*;


#[component]
pub fn Content(children: Option<Element>) -> Element {
    rsx! {
        main { class: "flex-1 bg-white p-8",
            match children {
                Some(c) => c,
                None => rsx!{}
            }
        }
    }
}