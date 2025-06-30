


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error")]
    DbError(sqlx::Error),

    #[error("Failed to read configuration file")]
    ConfigReadError,

    #[error("Failed to parse configuration file")]
    ConfigParseError,

    #[error("Invalid start or end time")]
    InvalidTime,

    #[error("Conflict record")]
    ConflictRecord(RecordConflictInfo),

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
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // TODO: this is not a good way to compare DB errors, but we don't do that in the code
            (Self::DbError(_), Self::DbError(_)) => true,
            (Self::InvalidTime, Self::InvalidTime) => true,
            (Self::ConflictRecord(v1), Self::ConflictRecord(v2)) => v1 == v2,
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
            sqlx::Error::Database(e) => {
                let err: &PgDatabaseError = e.downcast_ref();
                match (err.code(), err.schema(), err.table()) {
                    ("23P01", Some("rsvp"), Some("Records")) => {
                        Error::ConflictRecord(err.detail().unwrap().parse().unwrap())
                    }
                    _ => Error::DbError(sqlx::Error::Database(e)),
                }
            }
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::DbError(e),
        }
    }
}



