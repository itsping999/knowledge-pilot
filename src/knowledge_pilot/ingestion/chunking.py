"""Simple chunking utilities for the first RAG milestone."""

from __future__ import annotations

from knowledge_pilot.schema import Chunk, Document


def chunk_document(document: Document, *, chunk_size: int = 800) -> list[Chunk]:
    """Split a document into fixed-size chunks with stable ids."""

    text = document.text.strip()
    if not text:
        return []

    chunks: list[Chunk] = []
    start = 0
    index = 0
    while start < len(text):
        end = min(start + chunk_size, len(text))
        chunk_text = text[start:end].strip()
        if chunk_text:
            chunks.append(
                Chunk(
                    id=f"{document.id}:{index}",
                    text=chunk_text,
                    metadata={**document.metadata, "document_id": document.id},
                )
            )
        start = end
        index += 1
    return chunks

