use tokio::sync::watch::error;
use axum::response::IntoResponse;
use crate::models::web::Response;





#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("not implemented")]
    NotImplemented,

    #[error("Database error")]
    DbError(sqlx::Error),

    #[error("Failed to read configuration file")]
    ConfigReadError,

    #[error("Failed to parse configuration file")]
    ConfigParseError,

    #[error("Invalid start or end time")]
    InvalidTime,

    #[error("No record found by the given condition")]
    NotFound,

    #[error("Invalid record id: {0}")]
    InvalidRecordId(i64),

    #[error("Invalid page size: {0}")]
    InvalidPageSize(i64),

    #[error("Invalid cursor: {0}")]
    InvalidCursor(i64),

    #[error("Invalid status: {0}")]
    InvalidStatus(i32),

    #[error("unknown error")]
    Unknown,

    #[error("{0} value is empty")]
    EmptyValue(String),

    #[error("invalid project code: {0}")]
    InvalidProjectCode(String),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // TODO: this is not a good way to compare DB errors, but we don't do that in the code
            (Self::DbError(_), Self::DbError(_)) => true,
            (Self::InvalidTime, Self::InvalidTime) => true,
            (Self::NotFound, Self::NotFound) => true,
            (Self::InvalidRecordId(v1), Self::InvalidRecordId(v2)) => v1 == v2,
            (Self::Unknown, Self::Unknown) => true,
            _ => false,
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::DbError(e),
        }
    }
}


// Implement IntoResponse for Error so it can be used in Axum handlers
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        use axum::Json;
        use crate::models::web::Response as WebResponse;
        let body = Json(WebResponse::<()> {
            result: false,
            msg: self.to_string(),
            data: (),
        });
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}


