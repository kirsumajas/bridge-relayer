use anyhow::{anyhow, Context, Result};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as B64;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};

pub fn load_keypair_base64(sk_b64: &str) -> Result<Keypair> {
    let raw = B64.decode(sk_b64).context("invalid base64 for relayer secret")?;
    if raw.len() != 32 && raw.len() != 64 {
        return Err(anyhow!("expected 32 or 64 bytes secret, got {}", raw.len()));
    }
    // Accept either 32-byte seed or 64-byte secret key
    let secret = if raw.len() == 32 {
        SecretKey::from_bytes(&raw)?
    } else {
        SecretKey::from_bytes(&raw[..32])?
    };
    let public = PublicKey::from(&secret);
    Ok(Keypair { secret, public })
}

pub fn sign(hash32: &[u8; 32], kp: &Keypair) -> [u8; 64] {
    let sig: Signature = kp.sign(hash32);
    sig.to_bytes()
}
