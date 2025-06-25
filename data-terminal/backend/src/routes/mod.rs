
mod login;

use axum::Router;


pub fn router() -> Router {
    let router = Router::new()
        .nest("/api/v1", api_routes_v1());
    router
}

fn api_routes_v1() -> Router {
    Router::new().nest("/login", login::routes())
}