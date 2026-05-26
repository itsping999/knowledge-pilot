"""Small in-memory retriever for development and deterministic tests."""

from __future__ import annotations

import hashlib
import math
import re
from collections import Counter

from knowledge_pilot.retrieval.base import Embedder
from knowledge_pilot.schema import Chunk, RetrievedChunk

TOKEN_RE = re.compile(r"[a-zA-Z0-9_]+")


class HashEmbedder:
    """Hashing embedder with no external dependencies."""

    def __init__(self, dimensions: int = 256) -> None:
        self.dimensions = dimensions

    def embed(self, text: str) -> list[float]:
        vector = [0.0] * self.dimensions
        for token, count in Counter(_tokens(text)).items():
            index = _stable_index(token, self.dimensions)
            vector[index] += float(count)
        return _normalize(vector)


class InMemoryRetriever:
    """Cosine-similarity retriever over local chunks."""

    def __init__(self, embedder: Embedder | None = None) -> None:
        self.embedder = embedder or HashEmbedder()
        self._entries: list[tuple[Chunk, list[float]]] = []

    def add(self, chunks: list[Chunk]) -> None:
        for chunk in chunks:
            self._entries.append((chunk, self.embedder.embed(chunk.text)))

    def retrieve(self, query: str, *, top_k: int) -> list[RetrievedChunk]:
        query_vector = self.embedder.embed(query)
        scored = [
            RetrievedChunk(chunk=chunk, score=_cosine(query_vector, vector))
            for chunk, vector in self._entries
        ]
        scored.sort(key=lambda item: item.score, reverse=True)
        return scored[:top_k]


def _tokens(text: str) -> list[str]:
    return [match.group(0).lower() for match in TOKEN_RE.finditer(text)]


def _stable_index(token: str, dimensions: int) -> int:
    digest = hashlib.sha256(token.encode("utf-8")).digest()
    return int.from_bytes(digest[:4], "big") % dimensions


def _normalize(vector: list[float]) -> list[float]:
    length = math.sqrt(sum(value * value for value in vector))
    if length == 0:
        return vector
    return [value / length for value in vector]


def _cosine(left: list[float], right: list[float]) -> float:
    return sum(a * b for a, b in zip(left, right))
