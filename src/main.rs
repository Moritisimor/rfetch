use clap::Parser;
use serde_json::Value;

mod flags;
mod helpers;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flags = flags::Flags::parse();
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    if flags.json {
        serde_json::from_str::<Value>(&flags.body)?;

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
