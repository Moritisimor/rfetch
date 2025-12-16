use anyhow::bail;
use clap::Parser;
use serde_json::Value;

mod flags;
mod helpers;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let flags = flags::Flags::parse();
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    if flags.json {
        if let Err(e) = serde_json::from_str::<Value>(&flags.body) {
            bail!("JSON-Parse Error: {}", e)
        };

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json")
        );
    }

    for header in &flags.headers {
        if header.is_empty() { continue }
        let (k, v) = match header.split_once(':') {
            Some(kv) => kv,
            None => bail!("Invalid header format (expected 'key: value').")
        };

        headers.insert(
            reqwest::header::HeaderName::from_bytes(k.as_bytes())?,
            reqwest::header::HeaderValue::from_bytes(v.as_bytes())?
        );
    }

    let response = match client
        .request(flags.extract_method()?, &flags.url)
        .body(flags.body.clone())
        .headers(headers)
        .send()
        .await {
        Ok(r) => r,
        Err(e) => bail!("Failed to send request: {}", e)
    };

    helpers::print_response(response, flags.debug).await;
    Ok(())
}
