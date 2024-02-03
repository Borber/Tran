use anyhow::Result;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Client;
use serde::Serialize;

use crate::{common::CLIENT, config::CONFIG, lang, manager::mirror};

/// 翻译结果
#[derive(Debug, Serialize)]
pub struct TransVO {
    /// 是否为单词
    pub word: bool,
    /// 翻译结果
    pub trans: Option<String>,
    /// 词典
    pub dicts: Option<Vec<Dict>>,
}

#[derive(Debug, Serialize)]
pub struct Dict {
    pub pos: String,
    pub terms: Vec<String>,
}

pub async fn translate(content: &str) -> Result<TransVO> {
    // 翻译目标语言
    let lang = lang::lang(content);

    // 转换为 url 编码
    let content = utf8_percent_encode(content, NON_ALPHANUMERIC).to_string();

    let mode = CONFIG.lock().mode;
    let host = match mode {
        0 => mirror::one(),
        _ => "https://translate.googleapis.com".to_string(),
    };
    send(&CLIENT, host, &lang, &content).await
}

async fn send(client: &Client, host: String, lang: &str, content: &str) -> Result<TransVO> {
    let resp = client
        .get(format!("{}/translate_a/single?client=gtx&sl=auto&tl={}&dj=1&dt=t&dt=bd&dt=qc&dt=rm&dt=ex&dt=at&dt=ss&dt=rw&dt=ld&q=%22{}%22", host, lang, content))
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Encoding", "gzip")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // 识别单词状态, 检测是否有 dict
    if let Some(dict) = resp["dict"].as_array() {
        let dicts = dict
            .iter()
            .map(|item| {
                let pos = item["pos"].as_str().unwrap_or("解析失败");
                let terms = match item["terms"].as_array() {
                    Some(terms) => terms.to_owned(),
                    None => {
                        vec![]
                    }
                };
                Dict {
                    pos: pos.to_string(),
                    terms: terms
                        .iter()
                        .map(|x| x.as_str().unwrap_or("解析失败").to_string())
                        .collect(),
                }
            })
            .collect();

        let trans = TransVO {
            word: true,
            trans: None,
            dicts: Some(dicts),
        };
        Ok(trans)
    } else {
        let tran_list = match resp["sentences"].as_array() {
            Some(tran_list) => tran_list.to_owned(),
            None => {
                vec![]
            }
        };
        let trans = tran_list
            .into_iter()
            .map(|x| x["trans"].as_str().unwrap_or_default().to_string())
            .collect::<Vec<String>>()
            .join("");

        let trans = trans
            .trim()
            .trim_matches('"')
            .trim_matches('“')
            .trim_matches('”')
            .trim_matches('「')
            .trim_matches('」')
            .trim_matches('《')
            .trim_matches('》');
        let result = TransVO {
            word: false,
            trans: Some(trans.to_string()),
            dicts: None,
        };
        Ok(result)
    }
}
