use once_cell::sync::Lazy;
use reqwest::Client;

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .proxy(reqwest::Proxy::all("http://127.0.0.1:7890").unwrap())
        .build()
        .unwrap()
});
