use anyhow::Result;
use log::info;
use tokio::time::{sleep, Duration};

use crate::config::Cfg;

/// Subscribes to Solana devnet logs for your program (stubbed here).
pub async fn run(cfg: Cfg) -> Result<()> {
    info!("Solana watcher started (program: {})", cfg.sol_bridge_program);
    // TODO: Replace with websockets subscription to program logs and parse SOL→TON burns.
    loop {
        sleep(Duration::from_secs(15)).await;
        // keepalive log
        info!("Solana watcher heartbeat (ws: {})", cfg.sol_rpc_ws);
    }
}
