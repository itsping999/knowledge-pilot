# Architecture

KnowledgePilot keeps the first traditional RAG implementation modular so future Agentic RAG behavior can reuse the same capabilities. The service is designed as an embedded Rust application backed by SQLite.

## Traditional RAG Layer

```text
RagService
  -> Embedder
  -> Retriever
  -> Context builder
  -> Generator
  -> RagResponse with citations
```

The first implementation stores documents and chunks in SQLite, computes deterministic hash embeddings, and performs vector similarity in process. This keeps the first version dependency-light. The planned upgrade is a `sqlite-vec` retriever implementation behind the same `Retriever` trait.

## Future Agentic RAG Layer

```text
AgenticRagOrchestrator
  -> Planner
  -> Tools
  -> Retriever
  -> Evidence evaluator
  -> Answer generator
```

The orchestrator should call the existing `RagService` or its lower-level parts. New behavior belongs around the pipeline unless the underlying retrieval or generation contract is insufficient.

## Logging

Use traditional logs through `log` and `env_logger`. The project intentionally avoids tracing and OpenTelemetry until there is a concrete production need.

