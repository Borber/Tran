use anyhow::Result;
use serde_json::json;

pub async fn translate(context: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let resp = client
        .post(" https://api.deeplx.org/translate")
        .json(&json!({ "text": context, "source_lang": "en", "target_lang": "zh" }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let data = resp["data"].as_str().unwrap();

    Ok(data.to_string())
}
