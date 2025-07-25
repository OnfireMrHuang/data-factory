use dioxus::prelude::*;
use crate::routes::Route;
use crate::utils::{cookie, request::{create_client, HttpRequest, RequestBuilder}, error::RequestError};
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
                                        let client = create_client("http://localhost:3000");
                                        let req_config = RequestBuilder::new()
                                            .header("Content-Type", "application/json")
                                            .build();
                                        let resp = client.post("/api/v1/login", Some(req_config), serde_json::json!({ "username": username, "password": password })).await;
                                        match resp {
                                            Ok(response_text) => {
                                                info!("响应文本: {}", response_text);
                                                
                                                let json: serde_json::Value = serde_json::from_str(&response_text).unwrap_or_default();
                                                info!("响应JSON: {:?}", json);
                                                
                                                if json.get("result").and_then(|v| v.as_bool()).unwrap_or(false) {
                                                    info!("登录成功，检查Cookie...");
                                                    info!("当前所有Cookie: {}", cookie::get_browser_cookies());
                                                    
                                                    // 检查cookie是否设置成功
                                                    let has_token = cookie::has_cookie("token");
                                                    let has_project_code = cookie::has_cookie("project_code");
                                                    info!("token cookie存在: {}", has_token);
                                                    info!("project_code cookie存在: {}", has_project_code);
                                                    
                                                    if has_token {
                                                        info!("Cookie设置成功！");
                                                        navigator.replace(Route::Home {});
                                                    } else {
                                                        info!("Cookie设置失败！");
                                                        error_msg_signal.set("登录成功但Cookie设置失败".to_string());
                                                    }
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
                                            Err(e) => {
                                                info!("login error: {:?}", e);
                                                let error_message = match e {
                                                    RequestError::NetworkError(msg) => format!("网络错误: {}", msg),
                                                    RequestError::HttpError { status, message } => format!("HTTP错误 {}: {}", status, message),
                                                    RequestError::AuthenticationError(msg) => format!("认证错误: {}", msg),
                                                    RequestError::TimeoutError => "请求超时".to_string(),
                                                    _ => "网络错误，请稍后重试".to_string(),
                                                };
                                                error_msg_signal.set(error_message);
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
