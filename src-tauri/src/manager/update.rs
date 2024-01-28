use crate::common::CLIENT;

use anyhow::Result;
use serde_json::Value;

pub async fn check() -> Result<bool> {
    let package = CLIENT
        .get("https://fastly.jsdelivr.net/gh/Borber/tran@latest/package.json")
        .send()
        .await?
        .json::<Value>()
        .await?;
    package["version"]
        .as_str()
        .map(|v| v != env!("CARGO_PKG_VERSION"))
        .ok_or_else(|| anyhow::anyhow!("version not found"))
}
