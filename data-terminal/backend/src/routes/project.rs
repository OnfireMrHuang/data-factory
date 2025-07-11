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



