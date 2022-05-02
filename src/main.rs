use std::env;
use teloxide::prelude::*;
use teloxide::types::Recipient;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let chat_id = env::var("TELOXIDE_CHAT_ID")
        .expect("Define the chat id with: export TELOXIDE_CHAT_ID=<chat_id>");
    let mut message = "Done!";
    if args.len() > 1 {
        message = &args[1];
    }

    let bot = Bot::from_env().auto_send();

    let recepient = Recipient::ChannelUsername(chat_id);
    bot.send_message(recepient, message).await.unwrap();
}
