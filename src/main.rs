use anyhow::bail;
use clap::Parser;
use owo_colors::OwoColorize;
use serde_json::Value;

mod flags;
mod helpers;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let flags = flags::Flags::parse();
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    if flags.json {
        let b = match &flags.body {
            Some(s) => s,
            None => anyhow::bail!("Using the JSON feature requires a body to be set!"),
        };

        if let Err(e) = serde_json::from_str::<Value>(b) {
            bail!("{} {}", "JSON-Parse Error:".red(), e.red())
        };

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
    }

    for header in &flags.headers {
        if header.is_empty() {
            continue;
        }

        let (k, v) = match header.split_once(':') {
            Some(kv) => kv,
            None => bail!("Invalid header format (expected 'key:value').".red()),
        };

        headers.insert(
            reqwest::header::HeaderName::from_bytes(k.as_bytes())?,
            reqwest::header::HeaderValue::from_bytes(v.as_bytes())?,
        );
    }

    let response = match client
        .request(flags.extract_method()?, &flags.url)
        .body(flags.clone().body.unwrap_or(String::new()))
        .headers(headers)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            if flags.debug {
                println!("{:#?}", e)
            }
            if e.is_builder() {
                bail!("Invalid URL Scheme!".red())
            }
            bail!("{}", e.red())
        }
    };

    helpers::handle_response(response, &flags).await
}
