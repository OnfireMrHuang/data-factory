// use axum::{routing::{post, delete}, Router, http::StatusCode, Json, extract::{Path, State}};
// use shaku::HasProvider;
// use super::jwt::Claims;
// use crate::models::web::Response;
// use crate::models::project::Project;
// use crate::autofac;
// use crate::autofac::AutoFacModule;
// use crate::services::ProjectService;
// use std::sync::Arc;


// pub fn routes() -> Router {
    
//     let state = autofac::get_app_state();

//     Router::new()
//     .route("/add", post(add_project))
//     .route("/edit", post(edit_project))
//     .route("/{code}", delete(del_project))
//     .with_state(state)
// }

// async fn add_project(
//     claims: Claims,
//     // State(state): State<autofac::AppState>,
//     Json(payload): Json<Project>,
// ) -> (StatusCode, Json<Response<String>>) {
//     let project_service: Box<dyn ProjectService> = state
//         .get_auto_fac_module()
//         .provide().unwrap();

//     let ans = project_service.add_project(payload).await;
//     match ans {
//         Ok(id) => {
//             return (StatusCode::OK, Json(Response::<String>{
//                 result: true,
//                 msg: "".to_string(),
//                 data: id,
//             }))
//         }
//         Err(e) => {
//             return (StatusCode::OK, Json(Response::<String>{
//                 result: false,
//                 msg: e.to_string(),
//                 data: "".to_string(),
//             }))
//         }
//     }
// }


// async fn edit_project(
//     claims: Claims,
//     State(_state): State<autofac::AppState>,
//     Json(payload): Json<Project>,
// ) -> (StatusCode, Json<Response<String>>) {
//     let project_service: Box<dyn ProjectService> = _state
//         .get_auto_fac_module()
//         .provide().unwrap();

//     let ans = project_service.add_project(payload).await;
//     match ans {
//         Ok(id) => {
//             return (StatusCode::OK, Json(Response::<String>{
//                 result: true,
//                 msg: "".to_string(),
//                 data: id,
//             }))
//         }
//         Err(e) => {
//             return (StatusCode::OK, Json(Response::<String>{
//                 result: false,
//                 msg: e.to_string(),
//                 data: "".to_string(),
//             }))
//         }
//     }
// }

// async fn del_project(
//     claims: Claims,
//     State(_state): State<autofac::AppState>,
//     Path(code): Path<String>,
// ) -> (StatusCode, Json<Response<String>>) {
//     let project_service: Box<dyn ProjectService> = _state
//         .get_auto_fac_module()
//         .provide().unwrap();

//     let ans = project_service.del_project(code).await;
//     match ans {
//         Ok(_) => {
//             return (StatusCode::OK, Json(Response::<String>{
//                 result: true,
//                 msg: "".to_string(),
//                 data: "".to_string(),
//             }))
//         }
//         Err(e) => {
//             return (StatusCode::OK, Json(Response::<String>{
//                 result: false,
//                 msg: e.to_string(),
//                 data: "".to_string(),
//             }))
//         }
//     }
// }