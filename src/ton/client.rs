use anyhow::Result;
use reqwest::Client;

#[derive(Clone)]
pub struct TonClient {
    pub base: String,
    http: Client,
}

impl TonClient {
    pub fn new(base: String) -> Self {
        Self { base, http: Client::new() }
    }

    pub async fn ping(&self) -> Result<()> {
        let _ = self.http.get(format!("{}/status", self.base)).send().await.ok();
        Ok(())
    }
}
