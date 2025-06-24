use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    rsx! {
        div { class: "flex w-full h-screen",
            div { class: "w-1/2 bg-cover bg-center bg-no-repeat",
                img { src: asset!("/assets/login-bg.png"), class: "w-full h-full object-cover" }
            }
            div { class: "w-1/2 flex items-center justify-center",
                div { class: "card bg-base-100 shadow-xl w-96",
                    div { class: "card-body",
                        h2 { class: "card-title text-2xl font-bold mb-4", "Login" }
                        div { class: "form-control",
                            label { class: "label", "Username" }
                            input { class: "input input-bordered", r#type: "text", placeholder: "Enter username" }
                        }
                        div { class: "form-control mt-4",
                            label { class: "label", "Password" }
                            input { class: "input input-bordered", r#type: "password", placeholder: "Enter password" }
                        }
                        button { class: "btn btn-primary mt-6", "Login" }
                    }
                }
            }
        }
    }
}
