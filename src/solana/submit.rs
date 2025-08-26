use anyhow::{anyhow, Result};
use log::info;
use borsh::BorshSerialize;

use crate::{attestations::TonToSolAttestationV1, config::Cfg};

/// Submit a TON→SOL attestation to your Solana bridge program (stub).
/// Replace with real RPC call building an Instruction & Transaction.
pub async fn submit_ton_attestation(cfg: &Cfg, att: &TonToSolAttestationV1) -> Result<()> {
    let bytes = att.try_to_vec().map_err(|e| anyhow!("borsh serialize: {e}"))?;
    let hash = crate::attestations::domain_hash("TON→SOL_BRIDGE_V1", &bytes);

    info!(
        "Would submit attestation to {} ({} bytes, hash: 0x{})",
        cfg.sol_bridge_program,
        bytes.len(),
        hex::encode(hash)
    );
    Ok(())
}
