use anyhow::{anyhow, Result};

pub fn hex32(hexstr: &str) -> Result<[u8; 32]> {
    let mut out = [0u8; 32];
    let v = hex::decode(hexstr.trim())?;
    if v.len() != 32 { return Err(anyhow!("expected 32 bytes")); }
    out.copy_from_slice(&v);
    Ok(out)
}

pub fn now_ts() -> u64 {
    (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()) as u64
}
