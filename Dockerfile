FROM rust:1-slim-bookworm AS builder

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY static ./static

RUN cargo build --release --locked

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home --home-dir /var/lib/knowledge-pilot knowledge-pilot

COPY --from=builder /app/target/release/knowledge-pilot /usr/local/bin/knowledge-pilot

USER knowledge-pilot
WORKDIR /var/lib/knowledge-pilot

ENV KNOWLEDGE_PILOT_ADDR=0.0.0.0:8080 \
    KNOWLEDGE_PILOT_DB_PATH=/var/lib/knowledge-pilot/data/knowledge-pilot.db \
    KNOWLEDGE_PILOT_REQUEST_BODY_LIMIT_BYTES=2097152 \
    KNOWLEDGE_PILOT_CHUNK_SIZE=800 \
    KNOWLEDGE_PILOT_CHUNK_OVERLAP=120 \
    KNOWLEDGE_PILOT_EMBEDDING_PROVIDER=ollama \
    KNOWLEDGE_PILOT_EMBEDDING_BASE_URL=http://host.docker.internal:11434 \
    KNOWLEDGE_PILOT_EMBEDDING_PATH=/api/embed \
    KNOWLEDGE_PILOT_EMBEDDING_MODEL=qwen3-embedding:0.6b \
    KNOWLEDGE_PILOT_LLM_PROVIDER=extractive \
    KNOWLEDGE_PILOT_UI_ENABLED=true \
    RUST_LOG=info

VOLUME ["/var/lib/knowledge-pilot/data"]
EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD ["curl", "-fsS", "http://127.0.0.1:8080/health"]

ENTRYPOINT ["/usr/local/bin/knowledge-pilot"]
