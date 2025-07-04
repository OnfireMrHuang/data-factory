mod login;
// mod project;
mod jwt;

use axum::{
    Router
};
use tower_http::cors::{Any, CorsLayer};


pub fn router() -> Router {

    // 解决跨域问题以及报错问题
    Router::new()
        .nest("/api/v1", api_routes_v1())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
}

fn api_routes_v1() -> Router {

    // UnAuth
    let public_routes = Router::new()
        .nest("/login", login::routes());

    public_routes

    // JwtAuth

    // let protected_routes = Router::new()
    //     .nest("/project", project::routes());

    // // 合并两组路由
    // public_routes.merge(protected_routes)
}


