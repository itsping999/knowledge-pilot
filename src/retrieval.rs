use std::sync::Arc;

use crate::embedding::cosine;
use crate::models::RetrievedChunk;
use crate::store::SqliteStore;

pub trait Retriever: Send + Sync {
    fn retrieve(
        &self,
        question: &str,
        query_vector: &[f32],
        top_k: usize,
    ) -> rusqlite::Result<Vec<RetrievedChunk>>;
}

pub struct SqliteRetriever {
    store: Arc<SqliteStore>,
}

impl SqliteRetriever {
    pub fn new(store: Arc<SqliteStore>) -> Self {
        Self { store }
    }
}

impl Retriever for SqliteRetriever {
    fn retrieve(
        &self,
        _question: &str,
        query_vector: &[f32],
        top_k: usize,
    ) -> rusqlite::Result<Vec<RetrievedChunk>> {
        let mut scored: Vec<RetrievedChunk> = self
            .store
            .load_chunks_with_embeddings()?
            .into_iter()
            .map(|(chunk, vector)| RetrievedChunk {
                score: cosine(query_vector, &vector),
                chunk,
            })
            .collect();

        scored.sort_by(|left, right| right.score.total_cmp(&left.score));
        scored.truncate(top_k);
        Ok(scored)
    }
}
