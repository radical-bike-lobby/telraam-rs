use clap::Parser;

use telraam_rs::{client::TelraamClient, endpoint, response::Response};

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

    let client = TelraamClient::new(api_token)?;

    let response = client.send(&endpoint::Welcome)?;

    println!("msg = {}", response.status().message);
    Ok(())
}
