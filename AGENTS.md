# KnowledgePilot Agent Instructions

## Project Goal

KnowledgePilot starts as a traditional RAG project and must remain easy to evolve into Agentic RAG.

The first milestone is a reliable, testable RAG pipeline:

```text
question -> retrieval -> context building -> answer generation -> citations/logging
```

Future Agentic RAG work should add orchestration around this pipeline instead of rewriting the retrieval and generation layers.

## Engineering Rules

- Keep retrieval, context construction, and LLM generation as separate modules.
- Keep provider-specific code behind interfaces. Do not leak OpenAI, Ollama, Milvus, pgvector, or other vendor details into core pipeline objects.
- Preserve source metadata on every chunk: source path, title when available, section when available, and stable chunk id.
- Answers must be grounded in retrieved context and return citations.
- Prefer deterministic local tests with fake embedders, retrievers, and generators.
- Add Agentic behavior through `knowledge_pilot.agent` and `knowledge_pilot.tools`.
- Do not place secrets in the repo. Use environment variables and `.env.example` only.

## Git

- Use Conventional Commits, for example `feat(rag): add context builder`.
- Keep commits focused by module or behavior.

## Verification

Before handing off code changes, run:

```bash
python3 -m pytest
python3 -m compileall src
```
