use anyhow::Result;
use serde_json::Value;

use crate::common::CLIENT;

pub async fn update(old: &str) -> Result<bool> {
    let resp = CLIENT
        .get("https://cdn.jsdelivr.net/gh/Borber/tran/package.json")
        .send()
        .await?
        .json::<Value>()
        .await?;
    let version = resp["version"].as_str().unwrap_or_default();
    Ok(version != old)
}
