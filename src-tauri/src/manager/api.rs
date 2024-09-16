use anyhow::Result;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Client;
use serde::Serialize;

use crate::{common::CLIENT, config, lang, manager::mirror};

// TODO: 数组字段枚举
/// 翻译结果
#[derive(Debug, Clone, Serialize)]
pub struct TransVO {
    /// 是否为单词
    pub word: bool,
    /// 翻译结果
    pub trans: Option<Vec<Tran>>,
    /// 词典
    pub dicts: Option<Vec<Dict>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Dict {
    pub pos: String,
    pub terms: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tran {
    // 0: 文本
    // 1: 换行
    pub typ: u64,
    pub data: Option<String>,
}

pub async fn translate(content: &str) -> Result<TransVO> {
    // 翻译目标语言
    let lang = lang::lang(content);

    // 转换为 url 编码
    let content = utf8_percent_encode(content, NON_ALPHANUMERIC).to_string();

    // 将多行合并一行
    let content = content.replace(['\n', '\r'], " ");
    let host = if config::mode() {
        mirror::one()
    } else {
        "https://translate.googleapis.com".to_string()
    };
    send(&CLIENT, host, &lang, &content).await
}

async fn send(client: &Client, host: String, lang: &str, content: &str) -> Result<TransVO> {
    let resp = client
        .get(format!("{}/translate_a/single?client=gtx&sl=auto&tl={}&dj=1&dt=t&dt=bd&dt=qc&dt=rm&dt=ex&dt=at&dt=ss&dt=rw&dt=ld&q={}", host, lang, content))
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

        let mut result = vec![];
        for trans in tran_list {
            let trans = trans["trans"].as_str().unwrap_or_default();
            let mut tmp = String::new();

            for c in trans.chars() {
                match c {
                    '\r' => {
                        // Assuming '\r' is always followed by '\n' and they indicate a new 'Tran' type 1.
                        if !tmp.is_empty() {
                            result.push(Tran {
                                typ: 0,
                                data: Some(tmp.clone()),
                            });
                            tmp.clear();
                        }
                        result.push(Tran { typ: 1, data: None });
                    }
                    '\n' => (), // Do nothing assuming '\n' is always preceded by '\r', handled above.
                    _ => tmp.push(c),
                }
            }
            if !tmp.is_empty() {
                result.push(Tran {
                    typ: 0,
                    data: Some(tmp),
                });
            }
        }

        let result = TransVO {
            word: false,
            trans: Some(result),
            dicts: None,
        };
        Ok(result)
    }
}
