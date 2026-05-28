use crate::models::{Chunk, Document};

pub fn chunk_document(document: &Document, chunk_size: usize) -> Vec<Chunk> {
    let text = document.text.trim();
    if text.is_empty() {
        return Vec::new();
    }

    let mut chunks = Vec::new();
    let mut start = 0;
    let chars: Vec<char> = text.chars().collect();

    while start < chars.len() {
        let end = usize::min(start + chunk_size, chars.len());
        let chunk_text: String = chars[start..end].iter().collect();
        let chunk_text = chunk_text.trim().to_string();
        if !chunk_text.is_empty() {
            let chunk_index = chunks.len();
            chunks.push(Chunk {
                id: format!("{}:{}", document.id, chunk_index),
                document_id: document.id.clone(),
                chunk_index,
                text: chunk_text,
                source: document.source.clone(),
            });
        }
        start = end;
    }

    chunks
}

#[cfg(test)]
mod tests {
    use crate::models::Document;

    use super::chunk_document;

    #[test]
    fn chunk_ids_are_stable() {
        let document = Document {
            id: "doc".to_string(),
            title: "Doc".to_string(),
            source: "test".to_string(),
            text: "hello world".to_string(),
        };

        let chunks = chunk_document(&document, 5);

        assert_eq!(chunks[0].id, "doc:0");
        assert_eq!(chunks[1].id, "doc:1");
    }
}
