
use wasm_bindgen::JsCast;

// 获取浏览器 Cookie 的函数
pub fn get_browser_cookies() -> String {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookie = html_document.cookie().unwrap();
    return cookie;
}

// 检查特定cookie是否存在
pub fn has_cookie(name: &str) -> bool {
    let cookies = get_browser_cookies();
    let has = cookies.contains(&format!("{}=", name));
    
    // 更详细的调试信息
    if !has {
        let cookie_parts: Vec<&str> = cookies.split(';').collect();
        for (i, part) in cookie_parts.iter().enumerate() {
            let trimmed = part.trim();
        }
    }
    
    has
}

// 获取特定cookie的值
pub fn get_cookie_value(name: &str) -> Option<String> {
    let cookies = get_browser_cookies();
    let cookie_parts: Vec<&str> = cookies.split(';').collect();
    
    for part in cookie_parts {
        let trimmed = part.trim();
        if trimmed.starts_with(&format!("{}=", name)) {
            let value = trimmed.split('=').nth(1)?;
            return Some(value.to_string());
        }
    }
    None
}