use axum::{routing::{post, delete}, Router, http::StatusCode, Json, extract::{Path, State}};
use shaku::HasProvider;
use super::jwt::Claims;
use crate::models::web;
use crate::models::project::Project;
use crate::autofac;
use crate::autofac::AutoFacModule;
use crate::services::ProjectService;
use std::sync::Arc;


pub fn routes() -> Router {
    Router::new()
        .route("/add", post(add_project))
        .route("/edit", post(edit_project))
        .route("/{code}", delete(del_project))
}


#[axum::debug_handler]
async fn add_project(claims: Claims, Json(project): Json<Project>) -> web::Response<()> {
    let project_service = autofac::get_project_service_global();
    let result = project_service.add_project(project).await;
    match result {
        Ok(_) => web::Response {
            result: true,
            msg: "Project added successfully".to_string(),
            data: (),
        },
        Err(e) => web::Response {
            result: false,
            msg: e.to_string(),
            data: (),
        },
    }
}

#[axum::debug_handler]
async fn edit_project(claims: Claims, Json(project): Json<Project>) -> web::Response<()> {
    let project_service = autofac::get_project_service_global();
    let result = project_service.update_project(project).await;
    match result {
        Ok(_) => web::Response {
            result: true,
            msg: "Project updated successfully".to_string(),
            data: (),
        },
        Err(e) => web::Response {
            result: false,
            msg: e.to_string(),
            data: (),
        },
    }
}


#[axum::debug_handler]
async fn del_project(claims: Claims, Path(code): Path<String>) -> web::Response<()> {
    let project_service = autofac::get_project_service_global();
    let result = project_service.del_project(code).await;
    match result {
        Ok(_) => web::Response {
            result: true,
            msg: "Project deleted successfully".to_string(),
            data: (),
        },
        Err(e) => web::Response {
            result: false,
            msg: e.to_string(),
            data: (),
        },
    }
}


