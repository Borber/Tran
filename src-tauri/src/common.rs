use std::sync::atomic::AtomicUsize;

use once_cell::sync::Lazy;
use reqwest::Client;

use crate::manager;

// 全局储存按下时间戳 原子类型
pub static TIME: AtomicUsize = AtomicUsize::new(0);

pub static CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub async fn init() {
    manager::mirror::init().await;
}
