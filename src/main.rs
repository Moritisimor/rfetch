use clap::Parser;

mod flags;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flags = flags::Flags::parse();
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    if flags.json {
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

    println!("{:?}", response); // Better formatted printing coming soon.
    Ok(())
}
