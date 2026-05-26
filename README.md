# KnowledgePilot

KnowledgePilot is a RAG project scaffold designed to start simple and stay ready for Agentic RAG.

The initial implementation is a traditional RAG pipeline:

```text
Question -> Retriever -> Context Builder -> Answer Generator
```

The extension point for Agentic RAG is an orchestration layer that can call the same pipeline, retrieval tools, and future external tools in multiple steps.

## Layout

```text
src/knowledge_pilot/
  pipeline.py          # traditional RAG pipeline
  schema.py            # shared data contracts
  retrieval/           # retriever interfaces and implementations
  generation/          # answer generator interfaces and implementations
  ingestion/           # document loading and chunking
  agent/               # future Agentic RAG orchestration
  tools/               # tool contracts reusable by agents
```

## Quick Start

```bash
python3 -m venv .venv
source .venv/bin/activate
python3 -m pip install --upgrade pip setuptools wheel
python3 -m pip install -e ".[dev]"
python3 -m pytest
```

Run the local demo:

```bash
python3 examples/local_demo.py
```

## Design Principles

- Traditional RAG first: make retrieval quality, chunk metadata, citations, and evaluation reliable.
- Agent-ready interfaces: expose retrieval and generation as reusable services.
- Provider isolation: keep model and storage providers behind small protocols.
- Evidence by default: answers carry citations back to source chunks.

## Agentic RAG Roadmap

1. Add query rewriting and hybrid retrieval.
2. Add reranking and context compression.
3. Add tool routing for vector search, SQL, logs, and APIs.
4. Add a planner that decomposes complex questions into subqueries.
5. Add evidence evaluation and multi-step retrieval loops.
6. Add audit logs for tool calls, citations, and final answers.
