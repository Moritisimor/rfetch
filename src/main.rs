use anyhow::bail;
use clap::Parser;
use owo_colors::OwoColorize;

mod flags;
mod helpers;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let flags = flags::Flags::parse();
    let client = reqwest::Client::new();
    let headers = helpers::make_headers(&flags).await?;

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
