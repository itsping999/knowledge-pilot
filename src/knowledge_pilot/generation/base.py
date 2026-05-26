"""Provider-neutral answer generation protocol."""

from __future__ import annotations

from typing import Protocol

from knowledge_pilot.schema import RetrievedChunk


class AnswerGenerator(Protocol):
    """Generates an answer from retrieved context."""

    def generate(self, question: str, context: list[RetrievedChunk]) -> str:
        """Return an answer grounded in context."""

