use borsh::{BorshDeserialize, BorshSerialize};
use sha2::{Digest, Sha256};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TonToSolAttestationV1 {
    pub ver: u8,                 // 1
    pub src_chain: u8,           // 0 = TON
    pub kind: u8,                // 1 = BURN_JETTON
    pub cfg_hash: [u8; 32],
    pub nonce: u64,
    pub jetton_minter_ton: [u8; 36], // wc + hash packed
    pub amount_raw: u128,
    pub decimals_ton: u8,
    pub dst_solana_pubkey: [u8; 32],
    pub min_sol_out: Option<u64>,
    pub deadline_ts: Option<u64>,
    pub tx_hash_ton: [u8; 32],
    pub lt_ton: u64,
    pub timestamp_ton: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct SolToTonAttestationV1 {
    pub ver: u8,                 // 1
    pub src_chain: u8,           // 1 = Solana
    pub kind: u8,                // 1 = BURN_SPL
    pub cfg_hash: [u8; 32],
    pub nonce: u64,
    pub spl_mint: [u8; 32],
    pub amount_raw: u128,
    pub dst_ton_addr: [u8; 36],  // wc + hash packed
    pub min_ton_out: Option<u128>,
    pub deadline_ts: Option<u64>,
    pub src_slot: u64,
    pub src_sig_prefix: Option<[u8; 64]>,
}

pub fn domain_hash(domain: &str, payload: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(domain.as_bytes());
    h.update(payload);
    h.finalize().into()
}
