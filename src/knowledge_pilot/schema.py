"""Shared contracts for RAG and future Agentic RAG workflows."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any


@dataclass(frozen=True)
class Document:
    """Raw document before chunking."""

    id: str
    text: str
    metadata: dict[str, Any] = field(default_factory=dict)


@dataclass(frozen=True)
class Chunk:
    """Searchable document chunk with source metadata."""

    id: str
    text: str
    metadata: dict[str, Any] = field(default_factory=dict)


@dataclass(frozen=True)
class RetrievedChunk:
    """A chunk returned by retrieval with a relevance score."""

    chunk: Chunk
    score: float


@dataclass(frozen=True)
class Citation:
    """Citation attached to an answer."""

    chunk_id: str
    source: str | None
    score: float


@dataclass(frozen=True)
class RagAnswer:
    """Final answer from a RAG pipeline."""

    question: str
    answer: str
    citations: list[Citation]
    context: list[RetrievedChunk]


@dataclass(frozen=True)
class ToolResult:
    """Generic tool result for future Agentic RAG orchestration."""

    name: str
    content: str
    metadata: dict[str, Any] = field(default_factory=dict)

