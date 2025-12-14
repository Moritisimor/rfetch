use clap::Parser;

mod flags;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flags = flags::Flags::parse();
    let client = reqwest::Client::new();
    flags.verbose_print(format!("> Sending request to {}", &flags.url));

    let mut headers = reqwest::header::HeaderMap::new();
    if flags.json {
        headers
            .insert(reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"));
    }

    let request = client
        .request(flags.extract_method()?, &flags.url)
        .headers(headers)
        .send()
        .await?;

    flags.verbose_print("= Successfully sent off request.");
    println!("{:?}", request); // Better formatted printing coming soon.
    Ok(())
}
