from knowledge_pilot.generation import ExtractiveAnswerGenerator
from knowledge_pilot.ingestion import chunk_document
from knowledge_pilot.pipeline import RagPipeline
from knowledge_pilot.retrieval import InMemoryRetriever
from knowledge_pilot.schema import Document


def test_pipeline_returns_answer_with_citation() -> None:
    document = Document(
        id="architecture",
        text="Retrieval and generation are separate modules for future agents.",
        metadata={"source": "test"},
    )
    retriever = InMemoryRetriever()
    retriever.add(chunk_document(document, chunk_size=200))

    pipeline = RagPipeline(
        retriever=retriever,
        generator=ExtractiveAnswerGenerator(),
        top_k=1,
    )

    result = pipeline.answer("Why separate retrieval and generation?")

    assert "Retrieval and generation" in result.answer
    assert result.citations[0].chunk_id == "architecture:0"
    assert result.citations[0].source == "test"

