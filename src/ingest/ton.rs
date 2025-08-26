use anyhow::{anyhow, Result};
use borsh::BorshSerialize;
use hex::FromHex;
use sqlx::SqlitePool;
use tracing::info;
use base64::Engine; // <-- add
use bs58;           // make sure bs58 is in Cargo.toml

use crate::attestations::{TonToSolAttestationV1, domain_hash};
use crate::repo::{insert_attestation, insert_burn, set_cursor};
use crate::ton::watcher::{poll_latest_burn, TonBurn};

fn cfg_hash_from_env() -> Result<[u8; 32]> {
    let s = std::env::var("CFG_HASH_HEX").unwrap_or_default();
    let bytes = <[u8; 32]>::from_hex(s).map_err(|e| anyhow!("CFG_HASH_HEX bad hex: {e}"))?;
    Ok(bytes)
}

fn ton_minter_wc_hash36_demo() -> [u8; 36] { [0u8; 36] }

fn dst_solana_pubkey_from_env() -> Result<[u8; 32]> {
    let s = std::env::var("DST_SOL_PUBKEY_BASE58")
        .map_err(|_| anyhow!("DST_SOL_PUBKEY_BASE58 missing"))?;
    let v = bs58::decode(s).into_vec().map_err(|e| anyhow!("dst sol pubkey bad base58: {e}"))?;
    if v.len() != 32 { return Err(anyhow!("dst sol pubkey must be 32 bytes")); }
    let mut k = [0u8; 32];
    k.copy_from_slice(&v);
    Ok(k)
}

fn b64_to_32(b64: &str) -> Result<[u8; 32]> {
    let v = base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|e| anyhow!("b64 decode: {e}"))?;
    if v.len() != 32 { return Err(anyhow!("expected 32 bytes after b64 decode")); }
    let mut a = [0u8; 32];
    a.copy_from_slice(&v);
    Ok(a)
}

pub async fn run(db: SqlitePool, tx: tokio::sync::broadcast::Sender<String>) -> Result<()> {
    let cfg_hash = cfg_hash_from_env()?;
    let minter36 = ton_minter_wc_hash36_demo();
    let dst_sol = dst_solana_pubkey_from_env()?;
    let decimals: u8 = std::env::var("JETTON_DECIMALS").ok().and_then(|s| s.parse().ok()).unwrap_or(9);

    loop {
        if let Some(burn) = poll_latest_burn().await? {
            let inserted = insert_burn(
                &db,
                &burn.tx_hash_b64,
                burn.lt as i64,
                burn.utime as i64,
                &burn.jetton_master_raw,
                &burn.owner_raw,
                &burn.jetton_wallet_raw,
                &burn.amount_raw,
                burn.custom_payload.as_deref(),
            ).await?;

            if inserted > 0 {
                let amount_raw_u128 = burn.amount_raw.parse::<u128>().unwrap_or(0);
                let tx_hash_32 = b64_to_32(&burn.tx_hash_b64)?;

                let att = TonToSolAttestationV1 {
                    ver: 1,
                    src_chain: 0,
                    kind: 1,
                    cfg_hash,
                    nonce: 0,
                    jetton_minter_ton: minter36,
                    amount_raw: amount_raw_u128,
                    decimals_ton: decimals,
                    dst_solana_pubkey: dst_sol,
                    min_sol_out: None,
                    deadline_ts: None,
                    tx_hash_ton: tx_hash_32,
                    lt_ton: burn.lt,
                    timestamp_ton: burn.utime,
                };

                let bytes = att.try_to_vec()?; // works once BorshSerialize is derived
                let h = domain_hash("TON→SOL_BRIDGE_V1", &bytes);
                let hhex = hex::encode(h);

                let _att_id = insert_attestation(&db, "TON_TO_SOL", &bytes, &hhex, Some(&burn.tx_hash_b64)).await?;

                if let Err(e) = crate::solana::submit::submit_ton_attestation(&crate::config::Cfg::from_env()?, &att).await {
                    tracing::warn!("submit attestation failed: {e}");
                }

                let _ = tx.send(
                    serde_json::json!({
                        "type":"attestation",
                        "kind":"TON_TO_SOL",
                        "hash_hex": hhex,
                        "src_tx": burn.tx_hash_b64,
                        "lt": burn.lt,
                        "utime": burn.utime,
                        "amount_raw": burn.amount_raw
                    }).to_string()
                );

                set_cursor(&db, "ton_last_lt", &burn.lt.to_string()).await.ok();

                info!("ingested burn→attestation lt={} hash={}", burn.lt, hhex);
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
