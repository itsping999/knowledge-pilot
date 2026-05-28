# KnowledgePilot Agent Instructions

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
- Preserve source metadata on every chunk: source path, title when available, section when available, and stable chunk id.
- Answers must be grounded in retrieved context and return citations.
- Prefer deterministic local tests with fake embedders, retrievers, and generators.
- Add Agentic behavior through `knowledge_pilot.agent` and `knowledge_pilot.tools`.
- Do not place secrets in the repo. Use environment variables and `.env.example` only.
- Use traditional logging through `log`/`env_logger`; do not add tracing or OpenTelemetry unless explicitly requested.
- Use `serde` and `serde_json` for JSON. Rust has no standard-library JSON package.
- Minimize external dependencies and justify every new crate.

## Git

- Use Conventional Commits, for example `feat(rag): add context builder`.
- Keep commits focused by module or behavior.

## Verification

Before handing off code changes, run:

```bash
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```
