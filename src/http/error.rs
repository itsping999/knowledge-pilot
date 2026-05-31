use axum::Json;
use axum::http::StatusCode;
use log::error;
use serde::Serialize;

use crate::rag::RagError;

#[derive(Clone, Copy, Debug)]
pub enum ErrorCode {
    BadRequest,
    Unauthorized,
    NotFound,
    Database,
    Internal,
    EmbeddingProvider,
    GenerationProvider,
}

impl ErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BadRequest => "bad_request",
            Self::Unauthorized => "unauthorized",
            Self::NotFound => "not_found",
            Self::Database => "database_error",
            Self::Internal => "internal_error",
            Self::EmbeddingProvider => "embedding_provider_error",
            Self::GenerationProvider => "generation_provider_error",
        }
    }

    pub fn status(self) -> StatusCode {
        match self {
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Database => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EmbeddingProvider => StatusCode::BAD_GATEWAY,
            Self::GenerationProvider => StatusCode::BAD_GATEWAY,
        }
    }

    pub fn default_message(self) -> &'static str {
        match self {
            Self::BadRequest => "bad request",
            Self::Unauthorized => "invalid or missing API token",
            Self::NotFound => "not found",
            Self::Database => "database error",
            Self::Internal => "internal error",
            Self::EmbeddingProvider => "embedding provider error",
            Self::GenerationProvider => "generation provider error",
        }
    }
}

pub struct ApiError {
    code: ErrorCode,
    message: String,
}

impl ApiError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::BadRequest, message)
    }

    pub fn unauthorized() -> Self {
        Self::new(
            ErrorCode::Unauthorized,
            ErrorCode::Unauthorized.default_message(),
        )
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::NotFound, message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Internal, message)
    }
}

impl From<rusqlite::Error> for ApiError {
    fn from(error: rusqlite::Error) -> Self {
        error!("database error: {}", error);
        Self::new(ErrorCode::Database, ErrorCode::Database.default_message())
    }
}

impl From<RagError> for ApiError {
    fn from(error: RagError) -> Self {
        match error {
            RagError::Database(error) => error.into(),
            RagError::Embed(error) => {
                error!("embedding error: {}", error);
                Self::new(
                    ErrorCode::EmbeddingProvider,
                    ErrorCode::EmbeddingProvider.default_message(),
                )
            }
            RagError::Generate(error) => {
                error!("generation error: {}", error);
                Self::new(
                    ErrorCode::GenerationProvider,
                    ErrorCode::GenerationProvider.default_message(),
                )
            }
        }
    }
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(ErrorBody {
            code: self.code.as_str(),
            message: self.message,
        });
        (self.code.status(), body).into_response()
    }
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}
