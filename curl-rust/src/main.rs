use std::error::Error;

use clap::Parser;
use reqwest::{
    Method,
    header::{HeaderMap, HeaderName, HeaderValue},
};

mod errors;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = models::Args::parse();
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    for h in &args.header {
        let h = h.as_str();
        if let Some((k, v)) = h.split_once(':') {
            let key = HeaderName::from_bytes(k.trim().as_bytes())?;
            let value = HeaderValue::from_str(v.trim())?;
            headers.insert(key, value);
        }
    }

    let mut request = client
        .request(
            match args.method.to_uppercase().as_str() {
                "GET" => Method::GET,
                "POST" => Method::POST,
                "PUT" => Method::PUT,
                "PATCH" => Method::PATCH,
                "DELETE" => Method::DELETE,
                _ => Method::GET,
            },
            &args.url,
        )
        .headers(headers);

    if !args.data.is_empty() {
        let json_value: serde_json::Value = serde_json::from_str(&args.data)?;
        request = request
            .header("Content-Type", "application/json")
            .json(&json_value);
    }

    let res = request.send().await?;
    let body = res.text().await?;

    println!("{}", body);
    Ok(())
}
