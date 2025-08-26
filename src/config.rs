use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use base64::Engine; // <-- needed for .decode(...)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cfg {
    // Solana
    pub sol_rpc_http: String,
    pub sol_rpc_ws: String,
    pub sol_bridge_program: String,

    // TON
    pub ton_api_base: String,
    pub ton_api_key: Option<String>,
    pub ton_watch_addr_b64: String,

    // Relayer signer
    pub relayer_sk_base64: String,
    pub relayer_pubkey: [u8; 32],

    // misc
    pub cfg_hash_hex: String,
}

impl Cfg {
    pub fn from_env() -> Result<Self> {
        let sol_rpc_http = std::env::var("SOL_RPC_HTTP")?;
        let sol_rpc_ws = std::env::var("SOL_RPC_WS")?;
        let sol_bridge_program = std::env::var("SOL_BRIDGE_PROGRAM")?;

        let ton_api_base = std::env::var("TON_API_BASE")?;
        let ton_api_key = std::env::var("TONCENTER_API_KEY").ok();
        let ton_watch_addr_b64 = std::env::var("TON_WATCH_ADDR_B64")?;

        let relayer_sk_base64 = std::env::var("RELAYER_SK_BASE64")?;
        let kp = base64::engine::general_purpose::STANDARD
            .decode(relayer_sk_base64.as_bytes())
            .map_err(|e| anyhow!("RELAYER_SK_BASE64 decode: {e}"))?;
        if kp.len() != 64 {
            return Err(anyhow!("RELAYER_SK_BASE64 must be 64 bytes (ed25519 keypair)"));
        }
        let mut pubkey = [0u8; 32];
        pubkey.copy_from_slice(&kp[32..]);

        let cfg_hash_hex = std::env::var("CFG_HASH_HEX")?;

        Ok(Self {
            sol_rpc_http,
            sol_rpc_ws,
            sol_bridge_program,
            ton_api_base,
            ton_api_key,
            ton_watch_addr_b64,
            relayer_sk_base64,
            relayer_pubkey: pubkey,
            cfg_hash_hex,
        })
    }
}
