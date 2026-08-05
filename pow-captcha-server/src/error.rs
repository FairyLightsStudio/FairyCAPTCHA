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

impl From<AppError> for connectrpc::ConnectError {
    fn from(err: AppError) -> Self {
        use connectrpc::ErrorCode;
        match err {
            AppError::DatabaseError(e) => {
                connectrpc::ConnectError::new(ErrorCode::Internal, format!("Database error: {}", e))
            }
            AppError::ConfigError(e) => {
                connectrpc::ConnectError::new(ErrorCode::Internal, format!("Configuration error: {}", e))
            }
            AppError::NotFound(e) => connectrpc::ConnectError::new(ErrorCode::NotFound, e),
            AppError::InvalidArgument(e) => connectrpc::ConnectError::new(ErrorCode::InvalidArgument, e),
            AppError::Unauthorized(e) => connectrpc::ConnectError::new(ErrorCode::Unauthenticated, e),
            AppError::DeadlineExceeded(e) => {
                connectrpc::ConnectError::new(ErrorCode::DeadlineExceeded, e)
            }
            AppError::InternalServerError => {
                connectrpc::ConnectError::new(ErrorCode::Internal, "An internal server error occurred")
            }
        }
    }
}
