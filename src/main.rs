use std::{collections::HashMap};
use clap::{Parser};
use urlencoding::encode;
use colored::Colorize;

const ENDPOINT: &str = "https://bitfrog.dev/v1";

#[derive(Parser)]
#[command(version, about, long_about = "A command line tool for easily sending Bitfrog notifications")]
struct Args{
    /// The project token (will attempt to load env variable BITFROG_TOKEN if not specified)
    #[arg(short('t'), long("token"), env("BITFROG_TOKEN"))]
    token: String,

    /// Name of the channel (will default to the first channel)
    #[arg(short('c'), long("channel"))]
    channel: Option<String>,

    /// The notification message
    #[arg(short('m'), long("message"), default_value_t = String::from("This is a placeholder message!"))]
    message: String,

    /// The notification title
    #[arg(short('T'), long("title"), help("adawdawd\n"))]
    title: Option<String>,

    /// Disables server warning messages
    #[arg(short('w'), long("nowarning"))]
    no_warnings: bool
}

fn send(token: String, message: String, title: Option<String>, channel: Option<String>,
        warnings: bool){
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
    }else{
        let response_json = result.json::<HashMap<String, String>>()
        .expect("Failed to parse json response.");

        if response_json.contains_key("error") {
            let error = response_json.get("error").expect("");
            println!("{}: {error}", "server error".red());
        }
        if warnings && response_json.contains_key("warning") {
            let warning = response_json.get("warning").expect("");
            println!("{}{}", "server warning: ".yellow(), warning.yellow());
        }
    }
}

fn main() {
    let args = Args::parse();

    send(
        args.token, 
        encode(&args.message).into_owned(), 
        args.title, 
        args.channel,
        !args.no_warnings
    );
}