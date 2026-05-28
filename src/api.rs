use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::models::{Document, RagResponse};
use crate::rag::RagService;

#[derive(Clone)]
pub struct AppState {
    rag: Arc<RagService>,
}

pub fn build_router(rag: Arc<RagService>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/documents", post(create_document))
        .route("/rag/query", post(query_rag))
        .with_state(AppState { rag })
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

async fn create_document(
    State(state): State<AppState>,
    Json(request): Json<CreateDocumentRequest>,
) -> Result<Json<CreateDocumentResponse>, ApiError> {
    let id = request
        .id
        .unwrap_or_else(|| format!("doc-{}", current_millis()));
    let document = Document {
        id: id.clone(),
        title: request.title,
        source: request.source.unwrap_or_else(|| "local".to_string()),
        text: request.text,
    };

    let chunks = state.rag.add_document(document)?;
    info!("document indexed: id={} chunks={}", id, chunks);

    Ok(Json(CreateDocumentResponse {
        id,
        chunks,
        status: "indexed".to_string(),
    }))
}

async fn query_rag(
    State(state): State<AppState>,
    Json(request): Json<QueryRequest>,
) -> Result<Json<RagResponse>, ApiError> {
    let top_k = request.top_k.unwrap_or(5).clamp(1, 20);
    let response = state.rag.answer(&request.question, top_k)?;
    Ok(Json(response))
}

fn current_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Deserialize)]
struct CreateDocumentRequest {
    id: Option<String>,
    title: String,
    text: String,
    source: Option<String>,
}

#[derive(Serialize)]
struct CreateDocumentResponse {
    id: String,
    chunks: usize,
    status: String,
}

#[derive(Deserialize)]
struct QueryRequest {
    question: String,
    top_k: Option<usize>,
}

pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl From<rusqlite::Error> for ApiError {
    fn from(error: rusqlite::Error) -> Self {
        error!("database error: {}", error);
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "database error".to_string(),
        }
    }
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(ErrorResponse {
            error: self.message,
        });
        (self.status, body).into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
