# Embedded RAG Production Notes

KnowledgePilot is an embedded traditional RAG service. It stores documents, chunks, embeddings, and query history in SQLite. Production deployments run as a single binary or container with a persistent data directory.

KnowledgePilot exposes separate upload and question answering pages. The upload page is available at `/upload`, and the question answering page is available at `/qa`.

KnowledgePilot can require a bearer API token for document and RAG APIs. Health checks and static pages remain available for runtime supervision.
