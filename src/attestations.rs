use borsh::{BorshSerialize, BorshDeserialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct TonToSolAttestationV1 {
    pub ver: u8,
    pub src_chain: u8,
    pub kind: u8,
    pub cfg_hash: [u8; 32],
    pub nonce: u64,
    pub jetton_minter_ton: [u8; 36],
    pub amount_raw: u128,
    pub decimals_ton: u8,
    pub dst_solana_pubkey: [u8; 32],
    pub min_sol_out: Option<u64>,
    pub deadline_ts: Option<u64>,
    pub tx_hash_ton: [u8; 32],
    pub lt_ton: u64,
    pub timestamp_ton: u64,
}

pub fn domain_hash(domain: &str, msg: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(domain.as_bytes());
    h.update(msg);
    let mut out = [0u8; 32];
    out.copy_from_slice(&h.finalize());
    out
}
