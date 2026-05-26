"""Agentic RAG extension point.

This module intentionally stays thin in the initial scaffold. The traditional
RAG pipeline should mature first, then this layer can add planning, routing,
tool calls, and evidence evaluation.
"""

from __future__ import annotations

from knowledge_pilot.pipeline import RagPipeline
from knowledge_pilot.schema import RagAnswer


class AgenticRagOrchestrator:
    """Initial orchestrator that delegates to the traditional pipeline."""

    def __init__(self, pipeline: RagPipeline) -> None:
        self.pipeline = pipeline

    def answer(self, question: str) -> RagAnswer:
        return self.pipeline.answer(question)

