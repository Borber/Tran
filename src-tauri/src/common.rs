use once_cell::sync::Lazy;
use reqwest::Client;

use crate::manager;

pub static CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub async fn init() {
    manager::mirror::init().await;
}
