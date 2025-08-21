#![allow(clippy::result_large_err)]
mod config;
mod attestations;
mod signer;
mod utils;
mod ton;
mod solana;

use anyhow::Result;
use log::{error, info, warn};
use tokio::{select, signal, sync::mpsc};

use crate::attestations::TonToSolAttestationV1;
use crate::config::Cfg;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cfg = Cfg::from_env()?;
    info!("bridge-relayer starting…");

    // Channels
    let (tx_att, mut rx_att) = mpsc::channel::<TonToSolAttestationV1>(1024);

    // Spawn watchers
    let cfg_ton = cfg.clone();
    tokio::spawn(async move {
        if let Err(e) = ton::watcher::run(cfg_ton, tx_att).await {
            error!("TON watcher exited: {e:#}");
        }
    });

    let cfg_sol = cfg.clone();
    tokio::spawn(async move {
        if let Err(e) = solana::watcher::run(cfg_sol).await {
            error!("Solana watcher exited: {e:#}");
        }
    });

    // Main loop: consume TON→SOL attestations and submit to Solana
    loop {
        select! {
            maybe_att = rx_att.recv() => {
                if let Some(att) = maybe_att {
                    info!("Received TON→SOL attestation candidate: {:?}", att);
                    if let Err(e) = solana::submit::submit_ton_attestation(&cfg, &att).await {
                        error!("Submit to Solana failed: {e:#}");
                    }
                } else {
                    warn!("Attestation channel closed");
                    break;
                }
            }
            _ = signal::ctrl_c() => {
                warn!("Ctrl-C, shutting down");
                break;
            }
        }
    }

    Ok(())
}
