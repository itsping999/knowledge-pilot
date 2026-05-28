use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub trait Embedder: Send + Sync {
    fn embed(&self, text: &str) -> Vec<f32>;
}

#[derive(Clone, Debug)]
pub struct HashEmbedder {
    dimensions: usize,
}

impl Default for HashEmbedder {
    fn default() -> Self {
        Self { dimensions: 256 }
    }
}

impl Embedder for HashEmbedder {
    fn embed(&self, text: &str) -> Vec<f32> {
        let mut vector = vec![0.0; self.dimensions];

        for token in tokenize(text) {
            let index = stable_index(&token, self.dimensions);
            vector[index] += 1.0;
        }

        normalize(vector)
    }
}

fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|part| !part.is_empty())
        .map(|part| part.to_lowercase())
        .collect()
}

fn stable_index(token: &str, dimensions: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    (hasher.finish() as usize) % dimensions
}

fn normalize(mut vector: Vec<f32>) -> Vec<f32> {
    let length = vector.iter().map(|value| value * value).sum::<f32>().sqrt();
    if length > 0.0 {
        for value in &mut vector {
            *value /= length;
        }
    }
    vector
}

pub fn cosine(left: &[f32], right: &[f32]) -> f32 {
    left.iter()
        .zip(right.iter())
        .map(|(left, right)| left * right)
        .sum()
}

pub fn encode_vector(vector: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(vector.len() * 4);
    for value in vector {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes
}

pub fn decode_vector(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Embedder, HashEmbedder, cosine};

    #[test]
    fn related_text_has_positive_score() {
        let embedder = HashEmbedder::default();
        let left = embedder.embed("traditional rag pipeline");
        let right = embedder.embed("rag pipeline");

        assert!(cosine(&left, &right) > 0.0);
    }
}
