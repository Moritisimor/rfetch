use anyhow::Error;
use clap::Parser;

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct Flags {
    #[arg(short, long)]
    pub url: String,

    #[arg(short, long, default_value = "GET")]
    pub method: String,

    #[arg(short, long)]
    pub body: Option<String>,

    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,

    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,

    #[arg(short, long, default_value = "false")]
    pub json: bool,

    #[arg(long, default_value = "false")]
    pub debug: bool,
}

impl Flags {
    pub fn extract_method(&self) -> Result<reqwest::Method, Error> {
        match &*self.method.to_lowercase() {
            "post" => Ok(reqwest::Method::POST),
            "get" => Ok(reqwest::Method::GET),
            "put" => Ok(reqwest::Method::PUT),
            "delete" => Ok(reqwest::Method::DELETE),
            "head" => Ok(reqwest::Method::HEAD),
            "options" => Ok(reqwest::Method::OPTIONS),
            "trace" => Ok(reqwest::Method::TRACE),
            "connect" => Ok(reqwest::Method::CONNECT),
            "patch" => Ok(reqwest::Method::PATCH),
            _ => anyhow::bail!("Invalid method."),
        }
    }
}
