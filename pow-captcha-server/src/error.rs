use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Authentication failed: {0}")]
    Unauthorized(String),

    #[error("Deadline exceeded: {0}")]
    DeadlineExceeded(String),

    #[error("Internal server error")]
    InternalServerError,
}

impl From<AppError> for volo_grpc::Status {
    fn from(err: AppError) -> Self {
        match err {
            AppError::DatabaseError(e) => volo_grpc::Status::internal(format!("Database error: {}", e)),
            AppError::ConfigError(e) => volo_grpc::Status::internal(format!("Configuration error: {}", e)),
            AppError::NotFound(e) => volo_grpc::Status::not_found(e),
            AppError::InvalidArgument(e) => volo_grpc::Status::invalid_argument(e),
            AppError::Unauthorized(e) => volo_grpc::Status::unauthenticated(e),
            AppError::DeadlineExceeded(e) => volo_grpc::Status::deadline_exceeded(e),
            AppError::InternalServerError => volo_grpc::Status::internal("An internal server error occurred"),
        }
    }
}
