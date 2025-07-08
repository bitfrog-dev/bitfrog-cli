use std::{collections::HashMap, process::exit};
use clap::Parser;
use urlencoding::encode;
use std::env;
use colored::Colorize;

const ENDPOINT: &str = "https://bitfrog.dev/v1";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args{
    /// The project token (will attempt to load env variable BITFROG_TOKEN if not specified)
    #[arg(short('t'), long("token"))]
    token: Option<String>,

    /// Name of the channel (will default to the first channel)
    #[arg(short('c'), long("channel"))]
    channel: Option<String>,

    /// The notification message
    #[arg(short('m'), long("message"), default_value_t = String::from("This is a placeholder message!"))]
    message: String,

    /// The notification title
    #[arg(short('T'), long("title"))]
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

    let token: String;

    match args.token {
        Some(arg_token) => { token = arg_token},
        None => {
            let val = env::var("BITFROG_TOKEN");
            match val {
                Ok(val) => { token = val },
                Err(_) => {
                    println!("{} BITFROG_TOKEN env variable not found, please specify a token with '{}'", "error:".red(), "--token <TOKEN>".yellow());
                    println!("\nFor more information, try '{}'.", "--help".bold());
                    exit(101);
                },
            }
        },
    }

    send(
        token, 
        encode(&args.message).into_owned(), 
        args.title, 
        args.channel
    );
}