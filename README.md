# KnowledgePilot

KnowledgePilot is a lightweight embedded RAG service designed to start with traditional RAG and stay ready for Agentic RAG.

Current stack:

```text
Rust
Axum
SQLite
reqwest
serde / serde_json
log / env_logger
```

The planned file-database vector upgrade path is `sqlite-vec`. The first implementation keeps vector search in process so the service can be tested without native SQLite extension loading.

The initial RAG flow is:

```text
Question -> Embedder -> Retriever -> Context Builder -> Answer Generator
```

The extension point for Agentic RAG is an orchestration layer that can call the same pipeline, retrieval tools, and future external tools in multiple steps.

## Layout

```text
src/
  main.rs              # server entrypoint
  api.rs               # Axum routes and handlers
  config.rs            # environment-based configuration
  db.rs                # SQLite connection and schema setup
  embedding.rs         # embedding abstraction and hash embedder
  generation.rs        # generator abstraction and local stub generator
  ingestion.rs         # document chunking
  models.rs            # shared contracts
  rag.rs               # traditional RAG service
  retrieval.rs         # retriever abstraction and SQLite-backed implementation
  store.rs             # SQLite persistence
```

## Quick Start

```bash
cargo test
cargo run
```

Query the API:

```bash
curl -s http://127.0.0.1:8080/health

curl -s -X POST http://127.0.0.1:8080/documents \
  -H 'content-type: application/json' \
  -d '{"title":"RAG roadmap","text":"KnowledgePilot starts with traditional RAG and keeps interfaces ready for Agentic RAG.","source":"demo"}'

curl -s -X POST http://127.0.0.1:8080/rag/query \
  -H 'content-type: application/json' \
  -d '{"question":"How does KnowledgePilot evolve?","top_k":3}'
```

## Design Principles

- Traditional RAG first: make retrieval quality, chunk metadata, citations, and evaluation reliable.
- Agent-ready interfaces: expose retrieval and generation as reusable services.
- Provider isolation: keep model and storage providers behind small traits.
- Evidence by default: answers carry citations back to source chunks.
- Embedded first: use SQLite and a local data directory before adding service databases.

## Agentic RAG Roadmap

1. Replace the in-process vector scan with sqlite-vec.
2. Add query rewriting and hybrid retrieval.
3. Add reranking and context compression.
4. Add tool routing for vector search, SQL, DingTalk docs, and APIs.
5. Add a planner that decomposes complex questions into subqueries.
6. Add evidence evaluation and multi-step retrieval loops.
7. Add audit logs for tool calls, citations, and final answers.
