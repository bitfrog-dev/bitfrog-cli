use std::collections::HashMap;
use clap::Parser;
use urlencoding::encode;

const ENDPOINT: &str = "https://bitfrog.dev/v1";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args{
    token: String,

    /// Name of the channel (will default to the first channel)
    #[arg(short('c'), long("channel"))]
    channel: Option<String>,

    /// The notification message
    #[arg(short('m'), long("message"), default_value_t = String::from("This is a placeholder message!"))]
    message: String,

    /// The notification title
    #[arg(short('t'), long("title"))]
    title: Option<String>,
}

fn send(token: String, message: String, title: Option<String>, channel: Option<String>){
    let mut url = ENDPOINT.to_owned() + "/notify?token=" + &token + "&message=" + &message;

    match title {
        Some(title) => {
            url += "&title=";
            url += &encode(&title);
        },
        None => {},
    }

    match channel {
        Some(channel) => {
            url += "&channel=";
            url += &encode(&channel);
        },
        None => {},
    }

    let result = reqwest::blocking::get(url)
    .expect("Failed to make request");

    if result.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
        println!("Too many requests. Try again later.");
    }else if !result.status().is_success() {
        let response_json = result.json::<HashMap<String, String>>()
                                                           .expect("Failed to parse json response.");
        let error = response_json.get("error").expect("Response has no error code.");
        println!("Bitfrog Error: {error}");
    }
}

fn main() {
    let args = Args::parse();
    // dbg!(args.token);
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);
    send(
        args.token, 
        encode(&args.message).into_owned(), 
        args.title, 
        args.channel
    );
}
