
use wasm_bindgen::JsCast;

// 获取浏览器 Cookie 的函数
pub fn get_browser_cookies() -> String {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie = html_document.cookie().unwrap();
    return cookie;
}