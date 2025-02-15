#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(" {0} ")]
    UnprocessableEntity(String),
    #[error(" {0} ")]
    EntityNotFound(String),
    #[error(" {0} ")]
    ValidationError(#[from] garde::Report),
    #[error("could'nt execute transaction")]
    TransactionError(#[source] sqlx::Error),
    #[error("an error occurred in operation on database")]
    SpecificOperationError(#[source] sqlx::Error),
    #[error("no rows affected: {0}")]
    NoRowsAffectedError(String),
    #[error(" {0} ")]
    KeyValueStoreError(#[from] redis::RedisError),
    #[error(" {0} ")]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error(" {0} ")]
    ConvertToUuidError(#[from] uuid::Error),
    #[error("authentication failed")]
    UnauthenticatedError,
    #[error("unauthorized")]
    UnauthorizedError,
    #[error("forbidden")]
    ForbiddenOperation,
    #[error(" {0} ")]
    ConversionEntityError(String),
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            AppError::UnprocessableEntity(_) => axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            AppError::EntityNotFound(_) => axum::http::StatusCode::NOT_FOUND,
            AppError::ValidationError(_) | AppError::ConvertToUuidError(_) => {
                axum::http::StatusCode::BAD_REQUEST
            }
            AppError::UnauthenticatedError | AppError::ForbiddenOperation => {
                axum::http::StatusCode::FORBIDDEN
            }
            AppError::UnauthorizedError => axum::http::StatusCode::UNAUTHORIZED,
            e @ (AppError::TransactionError(_)
            | AppError::SpecificOperationError(_)
            | AppError::NoRowsAffectedError(_)
            | AppError::KeyValueStoreError(_)
            | AppError::BcryptError(_)
            | AppError::ConversionEntityError(_)) => {
                tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "Unexpected error happennd",
                );
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        status_code.into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
