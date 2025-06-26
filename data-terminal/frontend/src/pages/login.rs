use dioxus::prelude::*;
use crate::routes::Route;
use dioxus::logger::tracing::info;

#[component]
pub fn Login() -> Element {

    let mut username_signal = use_signal(|| "".to_string());
    let mut password_signal = use_signal(|| "".to_string());

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
                                                    info!("login success, redirect to home page");
                                                    NavigationTarget::Internal(Route::Home {});
                                                } else {
                                                    let msg = json.get("msg").and_then(|v| v.as_str()).unwrap_or("Login failed");
                                                    // web_sys::window().unwrap().alert_with_message(msg).ok();
                                                    print!("{}", msg)
                                                }
                                            }
                                            Err(_) => {
                                                // 打印错误:
                                                print!(
                                                    "{}",
                                                    "登录失败，请检查用户名密码是否正确"
                                                )
                                                // web_sys::window().unwrap().alert_with_message("Network error").ok();
                                            }
                                        }
                                    });
                                },
                                "Login"
                            }
                        }
                    }
                }
            }
        }

    }
}
