use anyhow::{anyhow, Context, Result};
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine as _;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};

/// Load an Ed25519 keypair from base64:
/// - 64 bytes => standard ed25519 keypair bytes (secret||public)
/// - 32 bytes => secret only; public is derived
pub fn load_keypair_base64(sk_b64: &str) -> Result<Keypair> {
    let raw = B64.decode(sk_b64).context("invalid base64 for relayer secret")?;

    match raw.len() {
        64 => {
            let arr: [u8; 64] = raw
                .try_into()
                .map_err(|_| anyhow!("expected 64 bytes for keypair"))?;
            Keypair::from_bytes(&arr).context("invalid 64-byte ed25519 keypair")
        }
        32 => {
            let secret = SecretKey::from_bytes(&raw).context("invalid 32-byte secret")?;
            let public = PublicKey::from(&secret);
            Ok(Keypair { secret, public })
        }
        n => Err(anyhow!("expected 32 or 64 bytes secret, got {n}")),
    }
}

pub fn sign(hash32: &[u8; 32], kp: &Keypair) -> [u8; 64] {
    let sig: Signature = kp.sign(hash32);
    sig.to_bytes()
}

pub fn relayer_pubkey_hex(kp: &Keypair) -> String {
    hex::encode(kp.public.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;

    #[test]
    fn load_from_64_and_32() {
        // generate a keypair then test both encodings
        let kp = Keypair::generate(&mut OsRng);
        let mut raw64 = [0u8; 64];
        raw64[..32].copy_from_slice(kp.secret.as_bytes());
        raw64[32..].copy_from_slice(kp.public.as_bytes());

        let b64_64 = base64::engine::general_purpose::STANDARD.encode(raw64);
        let b64_32 = base64::engine::general_purpose::STANDARD.encode(kp.secret.as_bytes());

        let kpa = load_keypair_base64(&b64_64).unwrap();
        let kpb = load_keypair_base64(&b64_32).unwrap();
        assert_eq!(kpa.public.as_bytes(), kp.public.as_bytes());
        assert_eq!(kpb.public.as_bytes(), kp.public.as_bytes());
    }
}
