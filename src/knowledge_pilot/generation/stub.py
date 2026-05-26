"""Local answer generator used before wiring a real LLM provider."""

from __future__ import annotations

from knowledge_pilot.schema import RetrievedChunk


class ExtractiveAnswerGenerator:
    """Builds a deterministic answer from the best retrieved chunk."""

    def generate(self, question: str, context: list[RetrievedChunk]) -> str:
        if not context:
            return "I do not have enough retrieved context to answer."

        best = context[0]
        source = best.chunk.metadata.get("source", "unknown source")
        return (
            f"Based on {source}: {best.chunk.text.strip()} "
            f"(score={best.score:.3f})"
        )

