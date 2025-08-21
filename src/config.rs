use anyhow::{Context, Result};
use solana_sdk::pubkey::Pubkey;
use std::{env, str::FromStr};

#[derive(Clone, Debug)]
pub struct Cfg {
    pub sol_rpc_http: String,
    pub sol_rpc_ws: String,
    pub sol_bridge_program: Pubkey,

    pub ton_api_base: String,       // your TON indexer base
    pub ton_bridge_addr_b64: String,

    pub relayer_sk_base64: String,  // ed25519 secret (64 bytes) base64
    pub cfg_hash_hex: String,       // 32-byte hex for relayer set hash

    pub ton_confirms: u64,
    pub sol_finality_slots: u64,
}

impl Cfg {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            sol_rpc_http: env::var("SOL_RPC_HTTP")
                .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string()),
            sol_rpc_ws: env::var("SOL_RPC_WS")
                .unwrap_or_else(|_| "wss://api.devnet.solana.com/".to_string()),
            sol_bridge_program: Pubkey::from_str(&env::var("SOL_BRIDGE_PROGRAM")
                .context("SOL_BRIDGE_PROGRAM missing")?)?,
            ton_api_base: env::var("TON_API_BASE")
                .unwrap_or_else(|_| "https://testnet.toncenter.com/api/v3".to_string()),
            ton_bridge_addr_b64: env::var("TON_BRIDGE_ADDR_B64")
                .context("TON_BRIDGE_ADDR_B64 missing")?,
            relayer_sk_base64: env::var("RELAYER_SK_BASE64")
                .context("RELAYER_SK_BASE64 missing")?,
            cfg_hash_hex: env::var("CFG_HASH_HEX").unwrap_or_else(|_| "00".repeat(32)),
            ton_confirms: env::var("TON_CONFIRMS").ok().and_then(|s| s.parse().ok()).unwrap_or(8),
            sol_finality_slots: env::var("SOL_FINALITY_SLOTS").ok().and_then(|s| s.parse().ok()).unwrap_or(32),
        })
    }
}
