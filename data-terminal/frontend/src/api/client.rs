//! Shared HTTP client utilities for API calls

use crate::utils::cookie;
use crate::utils::request::{create_client_with_cookies, HttpClient};
use std::collections::HashMap;

/// Base URL for all API calls
/// TODO: Move this to environment configuration
pub const API_BASE_URL: &str = "http://localhost:3000";

/// Create an HTTP client with browser cookies automatically attached
pub fn create_api_client() -> HttpClient {
    let cookies_str = cookie::get_browser_cookies();
    let mut cookies = HashMap::new();

    // Parse cookies from browser
    for cookie_part in cookies_str.split(';') {
        let trimmed = cookie_part.trim();
        if let Some(eq_pos) = trimmed.find('=') {
            let name = &trimmed[..eq_pos];
            let value = &trimmed[eq_pos + 1..];
            cookies.insert(name.to_string(), value.to_string());
        }
    }

    create_client_with_cookies(API_BASE_URL, cookies)
}
