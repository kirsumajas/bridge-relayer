use anyhow::{anyhow, Result};
use log::info;

use crate::{attestations::TonToSolAttestationV1, config::Cfg};

/// Submit a TON→SOL attestation to your Solana bridge program (stub).
/// Replace with real RPC call building an Instruction & Transaction.
pub async fn submit_ton_attestation(cfg: &Cfg, att: &TonToSolAttestationV1) -> Result<()> {
    // Serialize (Borsh) and hash (domain-separated) if you want to log/debug here.
    let bytes = att.try_to_vec().map_err(|e| anyhow!("borsh serialize: {e}"))?;
    let hash = crate::attestations::domain_hash("TON→SOL_BRIDGE_V1", &bytes);

    // TODO: collect M-of-N signatures; build Instruction to your program; send via RpcClient
    info!(
        "Would submit attestation to {} ({} bytes, hash: 0x{})",
        cfg.sol_bridge_program,
        bytes.len(),
        hex::encode(hash)
    );
    Ok(())
}
