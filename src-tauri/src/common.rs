use std::sync::{atomic::AtomicBool, Arc};

use once_cell::sync::Lazy;
use reqwest::Client;

use crate::{config, manager};

pub static CLIENT: Lazy<Client> = Lazy::new(Client::new);
pub static PIN: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));

pub async fn init() {
    config::load();
    manager::mirror::init().await;
}
