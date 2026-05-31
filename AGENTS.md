# KnowledgePilot Agent Instructions

## Documentation Ownership

- Human-facing documentation belongs in `README.md`.
- Agent-facing implementation guidance, architecture boundaries, runtime conventions, and verification rules belong in `AGENTS.md`.
- Do not maintain parallel human docs under `docs/`, `openspec/`, `CHANGELOG.md`, `CONTRIBUTING.md`, or `SECURITY.md`; duplicate documentation drifts quickly.
- Markdown files under `eval/documents/` are test corpus content, not project documentation.
- When behavior changes, update `README.md` for user/operator impact and update `AGENTS.md` for codebase rules that future agents must follow.

## Current Project State

KnowledgePilot is currently a Rust 2024 embedded RAG service. It already exposes:

- `GET /health`
- `GET /documents`
- `POST /documents`
- `POST /documents/upload`
- `GET /documents/{id}`
- `GET /documents/{id}/chunks`
- `DELETE /documents/{id}`
- `POST /rag/query`
- `GET /rag/history`
- `GET /upload`
- `GET /qa`

The implemented pipeline is:

```text
document -> chunking -> configured embedding -> SQLite persistence
question -> configured embedding -> SQLite-backed in-process vector scan -> extractive answer -> citations
```

The first implementation stores documents, chunks, embeddings, and query history in SQLite. The default embedder remains deterministic 256-dimensional hash vectors stored as BLOBs, then compared in process with cosine similarity. `KNOWLEDGE_PILOT_EMBEDDING_PROVIDER=ollama` enables a local Ollama embedding model for higher-quality semantic retrieval while keeping the same embedded SQLite storage model.

Chunks preserve document title, source, nearest Markdown section, and stable chunk id. RAG responses return citations plus answer confidence. Citations return document title, section, source, chunk id, and score so the UI can show readable sources and highlight cited chunks in the original document modal.

RAG query history is stored in `rag_queries` and exposed through protected API `GET /rag/history`. Keep this endpoint operationally focused: bounded list retrieval, same bearer auth as document and RAG APIs, citation JSON parsed back into typed citation records, and confidence fields preserved for audit review.

Documents default to the `public` access scope. RAG queries include `public` plus any requested scopes, and retrieval must filter at the store/retriever boundary. Empty internal scope sets fail closed.

The default generator is extractive. `KNOWLEDGE_PILOT_LLM_PROVIDER=openai-compatible` enables a compatible `/chat/completions` provider when an API key is supplied. `custom` supports chat-completions-compatible local or third-party providers. `claude` supports Anthropic Messages format. LLMs are optional; without a usable LLM, the service remains traditional RAG.

## Project Goal

KnowledgePilot is a lightweight embedded RAG service. The current stack is Rust, Axum, SQLite, reqwest, serde/serde_json, and traditional log output through log/env_logger.

The first milestone is a reliable, testable RAG pipeline:

```text
question -> retrieval -> context building -> answer generation -> citations/logging
```

Future Agentic RAG work should add orchestration around this pipeline instead of rewriting the retrieval and generation layers. Keep sqlite-vec as the planned file-database vector retrieval upgrade path.

## Engineering Rules

- Keep retrieval, context construction, and LLM generation as separate modules.
- Keep provider-specific code behind interfaces. Do not leak OpenAI-compatible, Ollama, sqlite-vec, or other vendor details into core pipeline objects.
- Preserve source metadata on every chunk: source path or source id, title when available, section when available, and stable chunk id.
- Preserve and enforce access scope metadata for retrieval; restricted chunks must not be returned unless the query includes their scope.
- Answers must be grounded in retrieved context and return citations.
- Low-confidence answers must be explicit insufficient-context responses, not guessed answers. Preserve citations for manual checking.
- Prefer deterministic local tests with fake embedders, retrievers, and generators.
- Add Agentic behavior through `knowledge_pilot.agent` and `knowledge_pilot.tools`.
- Do not place secrets in the repo. Use environment variables and `.env.example` only.
- Use traditional logging through `log`/`env_logger`; do not add tracing or OpenTelemetry unless explicitly requested.
- Use `serde` and `serde_json` for JSON. Rust has no standard-library JSON package.
- Minimize external dependencies and justify every new crate.

## Module Boundaries

- `src/main.rs`: server bootstrap, logging init, command dispatch, config load, graceful shutdown.
- `src/app.rs`: dependency wiring for `RagService`.
- `src/commands.rs`: `eval` and `ingest` CLI workflows.
- `src/api.rs`: Axum routes and HTTP extraction.
- `src/config.rs`: environment-backed runtime config.
- `src/contracts.rs`: API request/response DTOs.
- `src/db.rs`: SQLite connection setup.
- `src/http/`: shared success envelope, error codes, and API-level error mapping.
- `src/store.rs`: SQLite schema and persistence only.
- `src/ingestion.rs`: document chunking and stable chunk ids.
- `src/embedding.rs`: `Embedder` trait, deterministic hash embedder, vector encoding helpers.
- `src/retrieval.rs`: `Retriever` trait and current SQLite-backed in-process vector scan.
- `src/generation.rs`: `Generator` trait and current extractive stub generator.
- `src/rag.rs`: traditional RAG orchestration and citation recording.
- `src/models.rs`: shared serializable contracts.

When adding new providers, put concrete provider details behind traits such as `Embedder`, `Retriever`, and `Generator`. Keep `RagService` focused on the pipeline.

## Runtime Configuration

Configuration is environment-driven:

```text
KNOWLEDGE_PILOT_ADDR=127.0.0.1:8080
KNOWLEDGE_PILOT_DB_PATH=./data/knowledge-pilot.db
KNOWLEDGE_PILOT_API_TOKEN=
KNOWLEDGE_PILOT_REQUEST_BODY_LIMIT_BYTES=2097152
KNOWLEDGE_PILOT_CHUNK_SIZE=800
KNOWLEDGE_PILOT_CHUNK_OVERLAP=120
KNOWLEDGE_PILOT_EMBEDDING_PROVIDER=ollama
KNOWLEDGE_PILOT_EMBEDDING_BASE_URL=http://127.0.0.1:11434
KNOWLEDGE_PILOT_EMBEDDING_PATH=/api/embed
KNOWLEDGE_PILOT_EMBEDDING_MODEL=qwen3-embedding:0.6b
KNOWLEDGE_PILOT_EMBEDDING_ALLOW_UNVERIFIED_MODEL=false
KNOWLEDGE_PILOT_RAG_MODE=auto
KNOWLEDGE_PILOT_LLM_PROVIDER=extractive
KNOWLEDGE_PILOT_UI_ENABLED=true
RUST_LOG=info
```

`.env.example` may document planned provider knobs, but code should only rely on environment variables it actually reads. Do not commit real API keys, tokens, cookies, or private service URLs.

Keep embedding and generation model guidance separate. `KNOWLEDGE_PILOT_EMBEDDING_MODEL` must be an embedding model such as an Ollama embedding model; optional `KNOWLEDGE_PILOT_LLM_MODEL` is only for a configured generator/planner and should not be presented as the primary retrieval model. Do not relax the Ollama embedding model validation for generic chat models. Use `KNOWLEDGE_PILOT_EMBEDDING_ALLOW_UNVERIFIED_MODEL=true` only as an explicit operator escape hatch for a verified custom embedding endpoint.

## Roadmap Constraints

- Keep sqlite-vec as the planned file-database vector retrieval upgrade path behind the `Retriever` trait.
- Prefer improving the current embedded baseline before adding external services.
- Add OpenAI-compatible, Ollama, Claude, or other providers behind generator/embedder/planner interfaces.
- Add Agentic RAG as orchestration around the existing retrieval and generation layers.
- Keep auditability through traditional logs, stored query history, and citations before introducing heavier observability.

## Git

- Use Conventional Commits, for example `feat(rag): add context builder`.
- Keep commits focused by module or behavior.

## Verification

Before handing off code changes, run:

```bash
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
KNOWLEDGE_PILOT_DB_PATH=/tmp/knowledge-pilot-eval-hash.db \
  KNOWLEDGE_PILOT_EMBEDDING_PROVIDER=hash \
  cargo run -- eval
KNOWLEDGE_PILOT_DB_PATH=/tmp/knowledge-pilot-eval-ollama.db \
  KNOWLEDGE_PILOT_EMBEDDING_PROVIDER=ollama \
  KNOWLEDGE_PILOT_EMBEDDING_MODEL=qwen3-embedding:0.6b \
  cargo run -- eval
KNOWLEDGE_PILOT_DB_PATH=/tmp/knowledge-pilot-ingest.db cargo run -- ingest README.md eval/documents
./scripts/smoke.sh
```

The eval command must validate retrieval, citation cleanliness, answer quality, and confidence behavior. The current fixture covers 51 enterprise documents and 101 cases. Cases in `eval/cases.json` may include `expected_answer_contains` and `expected_confidence_level`; treat missing phrases or wrong confidence levels as failures, not as cosmetic output differences. Unexpected citation sources fail a case unless `allow_additional_sources` is explicitly true for that case. Keep both hash and Ollama eval green when retrieval, citations, confidence, or answer formatting changes.

For API behavior changes, also run the service and smoke test:

```bash
cargo run
curl -s http://127.0.0.1:8080/health
curl -s -X POST http://127.0.0.1:8080/documents \
  -H 'content-type: application/json' \
  -d '{"title":"RAG roadmap","text":"KnowledgePilot starts with traditional RAG and keeps interfaces ready for Agentic RAG.","source":"demo"}'
curl -s -X POST http://127.0.0.1:8080/rag/query \
  -H 'content-type: application/json' \
  -d '{"question":"How does KnowledgePilot evolve?","top_k":3}'
curl -s http://127.0.0.1:8080/rag/history?limit=5
```
