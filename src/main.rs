use clap::Parser;
use std::env;
use teloxide::prelude::*;
use teloxide::types::InputFile;

/// Simple telegram communicator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Text to send
    #[clap(short, long, default_value = "Done!", global = true)]
    text: String,

    /// Flag to send a picture file
    #[clap(short, long)]
    photo: Option<String>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let args = Args::parse();

    let token =
        env::var("TELEGRAM_TOKEN").expect("Define the token with: export TELEGRAM_TOKEN=<token>");
    let chat_id = env::var("TELEGRAM_CHAT_ID")
        .expect("Define the chat id with: export TELEGRAM_CHAT_ID=<chat_id>");
    let bot = Bot::new(token).auto_send();

    match args.photo {
        Some(path) => {
            log::info!("Sending photo...");
            let photo = InputFile::file(path);
            bot.send_photo(chat_id, photo)
                .await
                .expect("Can not find the photo!");
            log::info!("Photo sent!");
        }
        None => {
            log::info!("Sending message...");
            bot.send_message(chat_id, &args.text).await.unwrap();
            log::info!("Message sent!");
        }
    }
}
