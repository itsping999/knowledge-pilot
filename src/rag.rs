use std::sync::Arc;

use crate::embedding::Embedder;
use crate::generation::Generator;
use crate::ingestion::chunk_document;
use crate::models::{Citation, Document, RagResponse};
use crate::retrieval::Retriever;
use crate::store::SqliteStore;

pub struct RagService {
    store: Arc<SqliteStore>,
    embedder: Arc<dyn Embedder>,
    retriever: Arc<dyn Retriever>,
    generator: Arc<dyn Generator>,
}

impl RagService {
    pub fn new(
        store: Arc<SqliteStore>,
        embedder: Arc<dyn Embedder>,
        retriever: Arc<dyn Retriever>,
        generator: Arc<dyn Generator>,
    ) -> Self {
        Self {
            store,
            embedder,
            retriever,
            generator,
        }
    }

    pub fn add_document(&self, document: Document) -> rusqlite::Result<usize> {
        self.store.save_document(&document)?;
        let chunks = chunk_document(&document, 800);
        for chunk in &chunks {
            let vector = self.embedder.embed(&chunk.text);
            self.store
                .save_chunk_with_embedding(chunk, &vector, "hash-256")?;
        }
        Ok(chunks.len())
    }

    pub fn answer(&self, question: &str, top_k: usize) -> rusqlite::Result<RagResponse> {
        let query_vector = self.embedder.embed(question);
        let context = self.retriever.retrieve(question, &query_vector, top_k)?;
        let answer = self.generator.generate(question, &context);
        let citations: Vec<Citation> = context
            .into_iter()
            .map(|item| Citation {
                document_id: item.chunk.document_id,
                chunk_id: item.chunk.id,
                source: item.chunk.source,
                score: item.score,
            })
            .collect();

        let citations_json = serde_json::to_string(&citations)
            .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))?;
        self.store
            .record_answer(question, &answer, &citations_json)?;

        Ok(RagResponse { answer, citations })
    }
}
