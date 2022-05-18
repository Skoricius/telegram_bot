use clap::Parser;
use std::env;
use teloxide::prelude::*;

/// Simple telegram communicator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Text to send
    #[clap(short, long, default_value = "Hey!")]
    text: String,

    /// Flag to send a picture file
    #[clap(short, long)]
    picture: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let message = args.text;

    let token =
        env::var("TELEGRAM_TOKEN").expect("Define the token with: export TELEGRAM_TOKEN=<token>");
    let chat_id = env::var("TELEGRAM_CHAT_ID")
        .expect("Define the chat id with: export TELEGRAM_CHAT_ID=<chat_id>");
    // let message = if args.len() > 1 {
    //     args[1..].join(" ")
    // } else {
    //     String::from("Done!")
    //     // String::from("Neist_point.jpg")
    // };
    if args.picture {
        println!("ey")
    } else {
        println!("yes")
    }

    pretty_env_logger::init();
    log::info!("Sending message...");
    let bot = Bot::new(token).auto_send();

    bot.send_message(chat_id.clone(), &message).await.unwrap();

    log::info!("Message sent...");
}
