use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Flags {
    #[arg(short, long)]
    pub url: String,

    #[arg(short, long, default_value = "GET")]
    pub method: String,

    #[arg(short, long, default_value = "")]
    pub body: String,
    
    #[arg(long, default_value = "")]
    pub head: Vec<String>,

    #[arg(short, long, default_value = "false")]
    pub json: bool,

    #[arg(long, default_value = "false")]
    pub debug: bool
}

impl Flags {
    pub fn extract_method(&self) -> Result<reqwest::Method, String> {
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
            _ => Err(format!("Invalid method: {}", self.method))
        }
    }
}
