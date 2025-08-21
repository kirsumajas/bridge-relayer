use anyhow::Result;
use log::{info, warn};
use tokio::{sync::mpsc::Sender, time::{sleep, Duration}};

use crate::{attestations::TonToSolAttestationV1, config::Cfg, utils::now_ts};

/// Polls TON indexer for jetton burns destined to your bridge, converts to attestations,
/// and sends them over the channel. This is a stub that emits a demo attestation periodically.
pub async fn run(cfg: Cfg, out: Sender<TonToSolAttestationV1>) -> Result<()> {
    let _client = super::client::TonClient::new(cfg.ton_api_base.clone());
    info!("TON watcher started (bridge addr: {})", cfg.ton_bridge_addr_b64);

    loop {
        // TODO: Replace with real polling & parsing of jetton burns + BurnBridgeMemo.
        sleep(Duration::from_secs(5)).await;

        let demo = TonToSolAttestationV1 {
            ver: 1,
            src_chain: 0,
            kind: 1,
            cfg_hash: crate::utils::hex32(&cfg.cfg_hash_hex).unwrap_or([0u8;32]),
            nonce: now_ts(),
            jetton_minter_ton: [0u8; 36],
            amount_raw: 123_000_000u128, // 0.123 (with 9 decimals) demo
            decimals_ton: 9,
            dst_solana_pubkey: [1u8; 32],
            min_sol_out: None,
            deadline_ts: None,
            tx_hash_ton: [2u8; 32],
            lt_ton: 0,
            timestamp_ton: now_ts(),
        };

        if let Err(e) = out.send(demo).await {
            warn!("TON watcher: channel closed: {e}");
            break;
        }
    }

    Ok(())
}
