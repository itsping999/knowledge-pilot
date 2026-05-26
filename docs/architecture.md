# Architecture

KnowledgePilot keeps the first traditional RAG implementation modular so future Agentic RAG behavior can reuse the same capabilities.

## Traditional RAG Layer

```text
RagPipeline
  -> Retriever
  -> AnswerGenerator
  -> RagAnswer with citations
```

The first implementation includes an in-memory retriever and a deterministic extractive generator. These are development defaults, not the final production providers.

## Future Agentic RAG Layer

```text
AgenticRagOrchestrator
  -> Planner
  -> Tools
  -> Retriever
  -> Evidence evaluator
  -> Answer generator
```

The orchestrator should call the existing `RagPipeline` or its lower-level parts. New behavior belongs around the pipeline unless the underlying retrieval or generation contract is insufficient.

