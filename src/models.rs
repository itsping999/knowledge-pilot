use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const PUBLIC_ACCESS_SCOPE: &str = "public";

pub fn default_access_scope() -> String {
    PUBLIC_ACCESS_SCOPE.to_string()
}

pub fn normalize_access_scope(scope: &str) -> Option<String> {
    let scope = scope.trim().to_ascii_lowercase();
    (!scope.is_empty()).then_some(scope)
}

pub fn allowed_access_scopes(scopes: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut allowed = Vec::new();

    for scope in std::iter::once(PUBLIC_ACCESS_SCOPE.to_string()).chain(
        scopes
            .iter()
            .filter_map(|scope| normalize_access_scope(scope)),
    ) {
        if seen.insert(scope.clone()) {
            allowed.push(scope);
        }
    }

    allowed
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub source: String,
    pub access_scope: String,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentSummary {
    pub id: String,
    pub title: String,
    pub source: String,
    pub access_scope: String,
    pub chunks: usize,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chunk {
    pub id: String,
    pub document_id: String,
    pub chunk_index: usize,
    pub document_title: String,
    pub section: String,
    pub text: String,
    pub source: String,
    pub access_scope: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RetrievedChunk {
    pub chunk: Chunk,
    pub score: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Citation {
    pub document_id: String,
    pub chunk_id: String,
    pub document_title: String,
    pub section: String,
    pub source: String,
    pub access_scope: String,
    pub score: f32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
}

impl ConfidenceLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "high" => Self::High,
            "low" => Self::Low,
            _ => Self::Medium,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnswerConfidence {
    pub level: ConfidenceLevel,
    pub score: f32,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RagResponse {
    pub answer: String,
    pub citations: Vec<Citation>,
    pub confidence: AnswerConfidence,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryHistoryEntry {
    pub id: i64,
    pub question: String,
    pub answer: String,
    pub citations: Vec<Citation>,
    pub confidence: AnswerConfidence,
    pub created_at: String,
}
