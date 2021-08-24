use reqwest::{blocking::Client, header};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    status: u32,
    code: u32,
    description: String,
    #[serde(default)]
    extra: String,
}

pub fn client(token: &str) -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    // let mut full = format!("https://mixin-api.zeromesh.net/{}", path).to_string();
    //reqwest::ClientBuilder::new()
    reqwest::blocking::Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Client::build()")
}
