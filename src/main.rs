use clap::Parser;
use serde_json::Value;

mod flags;
mod helpers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flags = flags::Flags::parse();
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    if flags.json {
        if let Err(e) = serde_json::from_str::<Value>(&flags.body) {
            return Err(e.into()); // Check if the supplied body is valid JSON.
        }

        headers
            .insert(reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"));
    }

    let response = client
        .request(flags.extract_method()?, &flags.url)
        .body(flags.body.clone())
        .headers(headers)
        .send()
        .await?;

    helpers::print_response(response, flags.debug).await;
    Ok(())
}
