use std::sync::{atomic::AtomicBool, Arc};

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use reqwest::Client;

use crate::{config, manager};

pub static CLIENT: Lazy<Client> = Lazy::new(Client::new);
pub static PIN: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));
pub static TMP_PIN: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));
pub static OLD: Lazy<Arc<RwLock<String>>> = Lazy::new(|| Arc::new(RwLock::new(String::new())));
pub static SIMULATION: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));

pub async fn init() {
    config::load();
    manager::mirror::init().await;
}
