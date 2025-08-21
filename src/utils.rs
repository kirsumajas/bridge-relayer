use anyhow::{anyhow, Result};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn hex32(hexstr: &str) -> Result<[u8; 32]> {
    let s = hexstr.trim_start_matches("0x");
    let v = hex::decode(s)?;
    if v.len() != 32 {
        return Err(anyhow!("expected 32 bytes, got {}", v.len()));
    }
    let mut a = [0u8; 32];
    a.copy_from_slice(&v);
    Ok(a)
}

pub fn now_ts() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
