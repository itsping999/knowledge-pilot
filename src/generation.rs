use crate::models::RetrievedChunk;

pub trait Generator: Send + Sync {
    fn generate(&self, question: &str, context: &[RetrievedChunk]) -> String;
}

pub struct ExtractiveGenerator;

impl Generator for ExtractiveGenerator {
    fn generate(&self, _question: &str, context: &[RetrievedChunk]) -> String {
        match context.first() {
            Some(best) => format!("Based on {}: {}", best.chunk.source, best.chunk.text.trim()),
            None => "I do not have enough retrieved context to answer.".to_string(),
        }
    }
}
