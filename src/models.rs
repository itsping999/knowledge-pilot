use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub source: String,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chunk {
    pub id: String,
    pub document_id: String,
    pub chunk_index: usize,
    pub text: String,
    pub source: String,
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
    pub source: String,
    pub score: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RagResponse {
    pub answer: String,
    pub citations: Vec<Citation>,
}
