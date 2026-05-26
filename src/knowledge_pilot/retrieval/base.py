"""Provider-neutral retrieval protocols."""

from __future__ import annotations

from typing import Protocol

from knowledge_pilot.schema import Chunk, RetrievedChunk


class Embedder(Protocol):
    """Converts text to vector-like numeric features."""

    def embed(self, text: str) -> list[float]:
        """Return an embedding for text."""


class Retriever(Protocol):
    """Retrieves chunks relevant to a query."""

    def add(self, chunks: list[Chunk]) -> None:
        """Add searchable chunks to the retriever."""

    def retrieve(self, query: str, *, top_k: int) -> list[RetrievedChunk]:
        """Return the top matching chunks."""

