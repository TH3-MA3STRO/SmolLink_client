use clap::Parser;
use reqwest::Client;
use std::collections::HashMap;

/// A program to shorten urls
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Link to shorten
    link: Option<String>,
    /// Assign a shorthand
    #[arg(short, long)]
    shorthand: Option<String>,
}
#[tokio::main]
async fn main() {
    let mut map: HashMap<&str, Option<String>> = HashMap::new();
    let http_client: Client = Client::new();
    let args = Args::parse();
    if args.link == None {
        println!("Please provide a link to shorten!");
        return ();
    }
    map.insert("link", args.link);
    if args.shorthand.is_some() {
        map.insert("shorthand", args.shorthand);
    } else {
        let default_val: Option<String> = Some(" ".to_owned());
        map.insert("shorthand", default_val);
    }
    //TODO: Retrieve the url from the environment variable, or configure the env variable if it is not set
    let url: String = "".to_string(); //ADD YOUR URL HERE
    let request: Result<String, reqwest::Error> = http_client
        .post(&url)
        .json(&map)
        .send()
        .await
        .unwrap()
        .text()
        .await;
    if request.is_ok() {
        let req_body = request.ok();
        if req_body.clone().unwrap().contains("Sorry") {
            println!("Error: {}", req_body.clone().unwrap());
            return ();
        } else {
            println!(
                "Here's the short url: {}/{}",
                &url,
                req_body.clone().unwrap()
            )
        };
    }
}
