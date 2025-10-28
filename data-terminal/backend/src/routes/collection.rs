use axum::{
    extract::{Path, Query, Extension},
    http::StatusCode,
    Json,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::collection::*;
use crate::services::collection_service::{CollectionService, ServiceError};

/// T043: Register collection routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(list_collection_tasks))
        .route("/", post(create_collection_task))
        .route("/:id", get(get_collection_task))
        .route("/:id", put(update_collection_task))
        .route("/:id", delete(delete_collection_task))
        .route("/:id/apply", post(apply_collection_task))
        .route("/generate-schema", post(generate_schema))
        // Note: Datasource routes should be under /datasources, not /collections
        // These are placeholders - actual implementation in datasource routes
}

/// Query parameters for list endpoint
#[derive(Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub status: Option<TaskStatus>,
    pub category: Option<CollectionCategory>,
    pub collect_type: Option<CollectType>,
}

/// Pagination response wrapper
#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Serialize)]
pub struct Pagination {
    pub page: i64,
    pub limit: i64,
    pub total: i64,
}

/// T036: POST /api/v1/collections - Create collection task
pub async fn create_collection_task(
    Extension(service): Extension<Arc<dyn CollectionService>>,
    Json(request): Json<CreateCollectTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let task = service.create_task(request).await?;
    Ok((StatusCode::CREATED, Json(task)))
}

/// T037: GET /api/v1/collections/:id - Get collection task by ID
pub async fn get_collection_task(
    Path(id): Path<String>,
    Extension(service): Extension<Arc<dyn CollectionService>>,
) -> Result<impl IntoResponse, AppError> {
    match service.get_task(&id).await? {
        Some(task) => Ok(Json(task)),
        None => Err(AppError::NotFound(format!("Task {} not found", id))),
    }
}

/// T038: PUT /api/v1/collections/:id - Update collection task
pub async fn update_collection_task(
    Path(id): Path<String>,
    Extension(service): Extension<Arc<dyn CollectionService>>,
    Json(request): Json<UpdateCollectTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let task = service.update_task(&id, request).await?;
    Ok(Json(task))
}

/// T039: POST /api/v1/collections/:id/apply - Apply collection task
pub async fn apply_collection_task(
    Path(id): Path<String>,
    Extension(service): Extension<Arc<dyn CollectionService>>,
) -> Result<impl IntoResponse, AppError> {
    let task = service.apply_task(&id).await?;
    Ok(Json(task))
}

/// T040: GET /api/v1/datasources/:id/tables - Get tables from datasource
pub async fn get_datasource_tables(
    Path(id): Path<String>,
    Extension(service): Extension<Arc<dyn CollectionService>>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: This should call DatasourceService, not CollectionService
    // For now, return empty array as placeholder
    let tables: Vec<TableMetadata> = vec![];
    Ok(Json(TableMetadataResponse { tables }))
}

/// T041: GET /api/v1/datasources/:id/tables/:tableName/fields - Get table fields
pub async fn get_table_fields(
    Path((datasource_id, table_name)): Path<(String, String)>,
    Extension(service): Extension<Arc<dyn CollectionService>>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: This should call DatasourceService, not CollectionService
    // For now, return empty array as placeholder
    let fields: Vec<FieldMetadata> = vec![];
    Ok(Json(fields))
}

/// T042: POST /api/v1/collections/generate-schema - Generate target schema
#[derive(Deserialize)]
pub struct GenerateSchemaRequest {
    pub datasource_id: String,
    pub resource_id: String,
    pub selected_tables: Vec<TableSelection>,
}

pub async fn generate_schema(
    Extension(service): Extension<Arc<dyn CollectionService>>,
    Json(request): Json<GenerateSchemaRequest>,
) -> Result<impl IntoResponse, AppError> {
    let schema = service
        .generate_schema(&request.datasource_id, &request.resource_id, request.selected_tables)
        .await?;

    Ok(Json(GenerateSchemaResponse {
        target_schema: schema,
    }))
}

/// T085 (Phase 7): GET /api/v1/collections - List all collection tasks
pub async fn list_collection_tasks(
    Query(query): Query<ListQuery>,
    Extension(service): Extension<Arc<dyn CollectionService>>,
) -> Result<impl IntoResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20).min(100);

    let (tasks, total) = service
        .list_tasks(page, limit, query.status, query.category, query.collect_type)
        .await?;

    let response = PaginatedResponse {
        data: tasks,
        pagination: Pagination { page, limit, total },
    };

    Ok(Json(response))
}

/// T086 (Phase 7): DELETE /api/v1/collections/:id - Delete collection task
pub async fn delete_collection_task(
    Path(id): Path<String>,
    Extension(service): Extension<Arc<dyn CollectionService>>,
) -> Result<impl IntoResponse, AppError> {
    service.delete_task(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Error handling
#[derive(Debug)]
pub enum AppError {
    ServiceError(ServiceError),
    NotFound(String),
}

impl From<ServiceError> for AppError {
    fn from(err: ServiceError) -> Self {
        AppError::ServiceError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::ServiceError(ServiceError::DatabaseError(msg)) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            AppError::ServiceError(ServiceError::NotFound(msg)) => (StatusCode::NOT_FOUND, msg),
            AppError::ServiceError(ServiceError::InvalidOperation(msg)) => {
                (StatusCode::FORBIDDEN, msg)
            }
            AppError::ServiceError(ServiceError::ValidationError(msg)) => {
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::ServiceError(ServiceError::ExternalServiceError(msg)) => {
                (StatusCode::BAD_GATEWAY, msg)
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
        };

        let body = Json(serde_json::json!({
            "error": status.canonical_reason().unwrap_or("Error"),
            "message": message,
        }));

        (status, body).into_response()
    }
}
