use dioxus::prelude::*;
use crate::routes::Route;
use dioxus::logger::tracing::info;
use async_std::task::sleep;

#[component]
pub fn Login() -> Element {

    let mut username_signal = use_signal(|| "".to_string());
    let mut password_signal = use_signal(|| "".to_string());
    let mut error_msg_signal = use_signal(|| "".to_string());
    let navigator = use_navigator(); 


    rsx! {
        div { class: "flex w-full h-screen",
            div { class: "w-1/2 bg-cover bg-center bg-no-repeat",
                img {
                    src: asset!("/assets/login-bg.png"),
                    class: "w-full h-full object-cover",
                }
            }
            div { class: "w-1/2 flex items-center justify-center",
                div { class: "card bg-base-100 shadow-xl w-96",
                    div { class: "card-body",
                        h2 { class: "card-title text-2xl font-bold mb-4", "Login" }
                        div { class: "form-control",
                            label { class: "label", "Username" }
                            input {
                                class: "input input-bordered",
                                r#type: "text",
                                placeholder: "Enter username",
                                value: "{username_signal}",
                                oninput: move |event| username_signal.set(event.value()),
                            }
                        }
                        div { class: "form-control mt-4",
                            label { class: "label", "Password" }
                            input {
                                class: "input input-bordered",
                                r#type: "password",
                                placeholder: "Enter password",
                                value: "{password_signal}",
                                oninput: move |event| password_signal.set(event.value()),
                            }
                            // 登录回调方法，单独抽出
                            button {
                                class: "btn btn-primary mt-6",
                                onclick: move |evt| {
                                    evt.prevent_default();
                                    let username = username_signal();
                                    let password = password_signal();
                                    spawn(async move {
                                        let resp = reqwest::Client::new().post("http://localhost:3000/api/v1/login")
                                            .timeout(std::time::Duration::from_secs(10))
                                            .header("Content-Type", "application/json")
                                            .json(&serde_json::json!({ "username": username, "password": password }))
                                            .send().await;
                                        match resp {
                                            Ok(r) => {
                                                let json: serde_json::Value = r.json().await.unwrap_or_default();
                                                if json.get("result").and_then(|v| v.as_bool()).unwrap_or(false) {
                                                    navigator.replace(Route::Home {});
                                                } else {
                                                    let msg = json.get("msg").and_then(|v| v.as_str()).unwrap_or("Login failed");
                                                    error_msg_signal.set(msg.to_string());
                                                    spawn({
                                                        async move {
                                                            sleep(std::time::Duration::from_secs(3)).await;
                                                            error_msg_signal.set("".to_string());
                                                        }
                                                    });
                                                }
                                            }
                                            Err(_) => {
                                                info!("login error!!!");
                                                error_msg_signal.set("网络错误，请稍后重试".to_string());
                                                spawn({
                                                    async move {
                                                        sleep(std::time::Duration::from_secs(3)).await;
                                                        error_msg_signal.set("".to_string());
                                                    }
                                                });
                                            }
                                        }
                                    });
                                },
                                "Login"
                            }
                        }
                        // 错误信息 label
                        {if !error_msg_signal().is_empty() {
                            Some(rsx!(
                                label { class: "text-red-500 mt-2", "{error_msg_signal}" }
                            ))
                        } else {
                            None
                        }}
                    }
                }
            }
        }
    }
}
