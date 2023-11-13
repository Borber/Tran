use anyhow::Result;
use urlencoding::encode;

use crate::common::CLIENT;

pub async fn translate(context: &str) -> Result<String> {
    // 转换为 url 编码
    let context = encode(context);
    let resp = CLIENT
        .get(format!("https://v2g.borber.top/translate_a/single?client=gtx&sl=auto&tl=zh-CN&dj=1&dt=t&dt=bd&dt=qc&dt=rm&dt=ex&dt=at&dt=ss&dt=rw&dt=ld&q=%22{}%22", &context))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let data = resp["sentences"][0]["trans"].as_str().unwrap();

    Ok(data.to_string())
}
