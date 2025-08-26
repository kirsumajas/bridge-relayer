use anyhow::{Context, Result};
use serde::Deserialize;
use tracing::info;
use base64::Engine; // <-- add

#[derive(Clone, Debug)]
pub struct TonBurn {
    pub tx_hash_b64: String,
    pub lt: u64,
    pub utime: u64,
    pub jetton_master_raw: String,
    pub owner_raw: String,
    pub jetton_wallet_raw: String,
    pub amount_raw: String,
    pub custom_payload: Option<Vec<u8>>,
}

#[derive(Deserialize, Debug)]
struct BurnsResponse {
    #[serde(rename = "jetton_burns", default)]
    burns: Vec<BurnItem>,
}

#[derive(Deserialize, Debug)]
struct BurnItem {
    #[serde(rename = "transaction_hash")] tx_hash_b64: String,
    #[serde(rename = "transaction_lt")]   transaction_lt: String,
    #[serde(rename = "transaction_now")]  transaction_now: u64,
    #[serde(rename = "jetton_master")]    jetton_master: String,
    #[serde(rename = "owner")]            owner: String,
    #[serde(rename = "jetton_wallet")]    jetton_wallet: String,
    #[serde(default)]                     amount: String,
    #[serde(default)]                     custom_payload: Option<String>,
}

fn b64_to_vec(s: &str) -> Option<Vec<u8>> {
    base64::engine::general_purpose::STANDARD.decode(s).ok()
}

pub async fn poll_latest_burn() -> Result<Option<TonBurn>> {
    let base = std::env::var("TON_API_BASE")
        .unwrap_or_else(|_| "https://testnet.toncenter.com/api/v3".into());
    let master = std::env::var("TON_WATCH_ADDR_B64")
        .map_err(|_| anyhow::anyhow!("TON_WATCH_ADDR_B64 missing"))?;
    let url = format!("{base}/jetton/burns?master={master}&limit=1&sort=desc");

    let mut req = reqwest::Client::new().get(&url);
    if let Ok(key) = std::env::var("TONCENTER_API_KEY") {
        if !key.is_empty() {
            req = req.header("X-API-KEY", key);
        }
    }

    info!("GET {}", url);
    let resp = req.send().await?;
    let status = resp.status();
    let text = resp.text().await?;
    info!(
        "status={} {} body[0..600]={}",
        status,
        status.canonical_reason().unwrap_or(""),
        &text[..text.len().min(600)]
    );

    if !status.is_success() {
        return Ok(None);
    }

    let parsed: BurnsResponse = serde_json::from_str(&text).context("parse burns json")?;
    let it = match parsed.burns.into_iter().next() {
        Some(x) => x,
        None => return Ok(None),
    };

    let lt: u64 = it.transaction_lt.parse().unwrap_or(0);
    let custom_payload = it.custom_payload.as_deref().and_then(b64_to_vec);

    let burn = TonBurn {
        tx_hash_b64: it.tx_hash_b64,
        lt,
        utime: it.transaction_now,
        jetton_master_raw: it.jetton_master,
        owner_raw: it.owner,
        jetton_wallet_raw: it.jetton_wallet,
        amount_raw: it.amount,
        custom_payload,
    };

    info!("TON watcher: forwarded burn tx {} amount {}", burn.tx_hash_b64, burn.amount_raw);
    Ok(Some(burn))
}
