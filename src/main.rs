mod api;
mod config;
mod db;
mod embedding;
mod generation;
mod ingestion;
mod models;
mod rag;
mod retrieval;
mod store;

use std::net::SocketAddr;
use std::sync::Arc;

use api::build_router;
use config::Config;
use db::open_database;
use embedding::HashEmbedder;
use generation::ExtractiveGenerator;
use log::info;
use rag::RagService;
use retrieval::SqliteRetriever;
use store::SqliteStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = Config::from_env();
    let db = open_database(&config.db_path)?;
    let store = Arc::new(SqliteStore::new(db));
    store.init()?;

    let rag = Arc::new(RagService::new(
        store.clone(),
        Arc::new(HashEmbedder::default()),
        Arc::new(SqliteRetriever::new(store.clone())),
        Arc::new(ExtractiveGenerator),
    ));

    let app = build_router(rag);
    let listener = tokio::net::TcpListener::bind(config.addr).await?;
    let addr: SocketAddr = listener.local_addr()?;
    info!("knowledge-pilot listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    if tokio::signal::ctrl_c().await.is_ok() {
        info!("shutdown signal received");
    }
}
