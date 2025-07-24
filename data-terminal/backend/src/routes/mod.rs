mod login;
mod project;
mod jwt;

use axum::{
    Router
};
use tower_http::cors::{Any, CorsLayer};
use axum::http::{HeaderName, Method}; 


pub fn router() -> Router {

    // 解决跨域问题以及报错问题
    Router::new()
        .nest("/api/v1", api_routes_v1())
        .layer(
            CorsLayer::new()
                .allow_origin("http://127.0.0.1:8080".parse::<axum::http::HeaderValue>().unwrap())
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS
                ])
                .allow_headers([
                    HeaderName::from_static("content-type"),
                    HeaderName::from_static("authorization"),
                    HeaderName::from_static("accept"),
                    HeaderName::from_static("origin"),
                    HeaderName::from_static("x-requested-with"),
                    HeaderName::from_static("cookie"),
                    HeaderName::from_static("credentials")
                ])
                .allow_credentials(true)  // 关键：允许携带凭证
                .expose_headers([
                    HeaderName::from_static("set-cookie"),
                    HeaderName::from_static("authorization")
                ])  // 暴露 set-cookie 头
        )
}

fn api_routes_v1() -> Router {

    // UnAuth
    let public_routes = Router::new()
        .nest("/login", login::routes());

    // JwtAuth
    let protected_routes = Router::new()
        .nest("/project", project::routes());

    // 合并两组路由
    public_routes.merge(protected_routes)
}


