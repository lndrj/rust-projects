use std::error::Error;

use clap::Parser;
use reqwest::{
    Method,
    header::{HeaderMap, HeaderName, HeaderValue},
};

use crate::errors::CurlErrors;

mod errors;
mod models;

#[tokio::main]
async fn main() -> Result<(), errors::CurlErrors> {
    let args = models::Args::parse();
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    for h in &args.header {
        let h = h.as_str();
        if let Some((k, v)) = h.split_once(':') {
            let key =
                HeaderName::from_bytes(k.trim().as_bytes()).map_err(|_| CurlErrors::HeaderError)?;
            let value = HeaderValue::from_str(v.trim()).map_err(|_| CurlErrors::HeaderError)?;
            headers.insert(key, value);
        } else {
            return Err(CurlErrors::HeaderError);
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
                _ => return Err(CurlErrors::BadMethod),
            },
            &args.url,
        )
        .headers(headers);

    if !args.data.is_empty() {
        let json_value: serde_json::Value =
            serde_json::from_str(&args.data).map_err(|_| CurlErrors::JsonError)?;
        request = request
            .header("Content-Type", "application/json")
            .json(&json_value);
    }

    let res = request
        .send()
        .await
        .map_err(|_| CurlErrors::ConnectionFailed)?;
    let body = res.text().await.map_err(|_| CurlErrors::ConnectionFailed)?;

    if &args.format.to_uppercase() == "JSON" {
        println!(
            "{:?}",
            serde_json::to_string_pretty(&body).map_err(|_| CurlErrors::JsonError)
        )
    } else if &args.format == "plain" {
        println!("{}", body);
    }
    Ok(())
}
