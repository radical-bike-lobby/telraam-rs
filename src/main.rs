use std::collections::HashMap;

use clap::Parser;
use reqwest::header;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 't', env = "TELRAAM_TOKEN")]
    telraam_token: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let api_token = &args.telraam_token as &str;

    let mut headers = header::HeaderMap::new();
    let mut api_token = header::HeaderValue::from_str(api_token)?;
    api_token.set_sensitive(true);
    headers.insert("X-Api-Key", api_token);

    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(APP_USER_AGENT)
        .default_headers(headers)
        .build()?;

    let resp = client
        .get("https://telraam-api.net/v1")
        .send()?
        .json::<HashMap<String, String>>()?;

    for (k, v) in resp {
        println!("{k} = {v}");
    }
    Ok(())
}
