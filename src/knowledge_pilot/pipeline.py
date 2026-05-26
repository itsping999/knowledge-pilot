"""Traditional RAG pipeline composed from replaceable parts."""

from __future__ import annotations

from knowledge_pilot.generation.base import AnswerGenerator
from knowledge_pilot.retrieval.base import Retriever
from knowledge_pilot.schema import Citation, RagAnswer, RetrievedChunk


class RagPipeline:
    """Retrieve context and generate a grounded answer."""

    def __init__(
        self,
        retriever: Retriever,
        generator: AnswerGenerator,
        *,
        top_k: int = 5,
    ) -> None:
        self.retriever = retriever
        self.generator = generator
        self.top_k = top_k

    def answer(self, question: str) -> RagAnswer:
        context = self.retriever.retrieve(question, top_k=self.top_k)
        answer_text = self.generator.generate(question, context)
        return RagAnswer(
            question=question,
            answer=answer_text,
            citations=self._citations(context),
            context=context,
        )

    @staticmethod
    def _citations(context: list[RetrievedChunk]) -> list[Citation]:
        return [
            Citation(
                chunk_id=item.chunk.id,
                source=item.chunk.metadata.get("source"),
                score=item.score,
            )
            for item in context
        ]

