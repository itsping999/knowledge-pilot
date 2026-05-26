from knowledge_pilot.generation import ExtractiveAnswerGenerator
from knowledge_pilot.ingestion import chunk_document
from knowledge_pilot.pipeline import RagPipeline
from knowledge_pilot.retrieval import InMemoryRetriever
from knowledge_pilot.schema import Document


def main() -> None:
    document = Document(
        id="rag-roadmap",
        text=(
            "KnowledgePilot starts with a traditional RAG pipeline. "
            "The pipeline keeps retrieval, context building, and answer "
            "generation separate so an Agentic RAG orchestrator can reuse them."
        ),
        metadata={"source": "examples/local_demo.py"},
    )

    retriever = InMemoryRetriever()
    retriever.add(chunk_document(document, chunk_size=160))

    pipeline = RagPipeline(
        retriever=retriever,
        generator=ExtractiveAnswerGenerator(),
        top_k=2,
    )
    result = pipeline.answer("How can KnowledgePilot evolve into Agentic RAG?")
    print(result.answer)
    for citation in result.citations:
        print(f"- {citation.chunk_id} {citation.source} {citation.score:.3f}")


if __name__ == "__main__":
    main()

