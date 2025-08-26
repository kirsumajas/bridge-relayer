mod config;
mod utils;
mod errors;
mod signer;
mod attestations;
mod repo;
mod api;
mod ingest;

mod solana;
mod ton;

use anyhow::Result;
use tracing::{info, Level};
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    init_tracing();

    let cfg = config::Cfg::from_env()?;

    info!("bridge-relayer starting…");
    info!("Relayer pubkey (hex): {}", hex::encode(cfg.relayer_pubkey));

    // DB
    let db = repo::init_db("sqlite://relayer.db").await?;

    // SSE broadcast
    let (tx, _rx) = tokio::sync::broadcast::channel::<String>(512);

    // spawn ingest (TON → attestation → submit)
    {
        let db_clone = db.clone();
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = ingest::ton::run(db_clone, tx_clone).await {
                tracing::error!("ingest.tn.run error: {e:?}");
            }
        });
    }

    // spawn Solana watcher (your existing heartbeat)
    tokio::spawn(async move {
        solana::watcher::run(&cfg).await.ok();
    });

    // start HTTP API (port from env or 3000)
    let bind = std::env::var("API_BIND").unwrap_or_else(|_| "127.0.0.1:3000".into());
    api::serve(bind, db, tx).await?;

    Ok(())
}

fn init_tracing() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info,tower_http=off".into());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_ansi(true)
        .with_max_level(Level::INFO)
        .init();
}
