use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

use crate::common::CLIENT;

/// 镜像
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mirror {
    pub urls: Vec<String>,
}

pub static MIRRORS: OnceCell<Mirror> = OnceCell::const_new();

// 初始化全局镜像
pub async fn init() {
    MIRRORS
        .get_or_init(|| async { get().await.unwrap_or(Mirror { urls: vec![] }) })
        .await;
}

// 从网络获取镜像地址
async fn get() -> Result<Mirror> {
    let mirror = match CLIENT
        .get("https://cdn.jsdelivr.net/gh/Borber/tran@master/resource/mirror.json")
        .send()
        .await
    {
        Ok(resp) => match resp.json::<Vec<String>>().await {
            Ok(urls) => Mirror { urls },
            Err(_) => Mirror { urls: vec![] },
        },
        Err(_) => Mirror { urls: vec![] },
    };
    Ok(mirror)
}

pub fn one() -> String {
    let mirrors = { MIRRORS.get().expect("Failed to get mirrors").urls.clone() };
    if mirrors.is_empty() {
        "https://v2g.borber.top".to_string()
    } else {
        let index = fastrand::usize(..mirrors.len());
        mirrors[index].to_owned().trim_end_matches('/').to_owned()
    }
}
