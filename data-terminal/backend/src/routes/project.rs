use axum::{routing::{post, delete}, Router, http::StatusCode, Json, extract::{Path, State}};
use shaku::HasProvider;
use shaku_axum::InjectProvided;
use super::jwt::Claims;
use crate::models::web::Response;
use crate::models::project::Project;
use crate::autofac;
use crate::autofac::AutoFacModule;
use crate::services::ProjectService;
use std::sync::Arc;


pub fn routes() -> Router {
    
    let state = autofac::get_app_state();

    Router::new()
    .route("/add", post(add_project))
    .route("/edit", post(edit_project))
    .route("/{code}", delete(del_project))
    .with_state(state)
}


async fn add_project(
    claims: Claims,
    State(state): State<autofac::AppState>,
    service: InjectProvided<AutoFacModule, dyn ProjectService>,
    Json(payload): Json<Project>,
) -> (StatusCode, Json<Response<String>>) {
    let module = Arc::<AutoFacModule>::from(&state);
    let service: Box<dyn ProjectService> = module.as_ref().provide().unwrap();
    let result: Result<String, crate::models::Error> = service.add_project(payload).await;
    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(Response::<String>{
            result: false,
            msg: result.err().unwrap().to_string(),
            data: "".to_string(),
        }));
    }
    (StatusCode::OK, Json(Response::<String>{
        result: true,
        msg: "".to_string(),
        data: result.unwrap(),
    }))
}


async fn edit_project(
    claims: Claims,
    State(_state): State<autofac::AppState>,
    Json(payload): Json<Project>,
) -> (StatusCode, Json<Response<String>>) {
    (StatusCode::OK, Json(Response::<String>{
        result: true,
        msg: "".to_string(),
        data: "".to_string(),
    }))
}

async fn del_project(
    claims: Claims,
    Path(code): Path<String>,
    State(_state): State<autofac::AppState>,
) -> (StatusCode, Json<Response<String>>) {
    (StatusCode::OK, Json(Response::<String>{
        result: true,
        msg: "".to_string(),
        data: "".to_string(),
    }))
}