use regex::Regex;

pub fn validate_code(code: &str) -> Option<String> {
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*$").unwrap();
    if code.is_empty() {
        Some("项目编码不能为空".to_string())
    } else if !re.is_match(code) {
        Some("仅支持英文、数字、下划线，且首字符必须为英文".to_string())
    } else {
        None
    }
}
