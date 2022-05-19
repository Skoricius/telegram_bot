use clap::Parser;
use reqwest::{Client, Error};
use std::env;
use std::fs::File;
use tokio::runtime::Runtime;

static API_URL: &str = "https://api.telegram.org/";

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

struct Chat {
    token: String,
    chat_id: String,
}

impl Chat {
    pub fn from_env() -> Self {
        let token = env::var("TELEGRAM_TOKEN")
            .expect("Define the token with: export TELEGRAM_TOKEN=<token>");
        let chat_id = env::var("TELEGRAM_CHAT_ID")
            .expect("Define the chat id with: export TELEGRAM_CHAT_ID=<chat_id>");
        Self { token, chat_id }
    }

    pub fn base_string(&self, method: &str) -> String {
        format!(
            "{}bot{}/{}?chat_id={}",
            API_URL, &self.token, &method, &self.chat_id
        )
    }
}

async fn send_message(client: &Client, chat: &Chat, message: &str) -> Result<(), Error> {
    let base_string = chat.base_string("sendMessage");

    client
        .post(base_string)
        .query(&[("text", message)])
        .send()
        .await?;
    Ok(())
}

fn send_photo(client: &Client, chat: Chat, path: &str) {
    let base_string = chat.base_string("sendMedia");
    let file = File::open(path).expect("Can not find the photo!");

    client.post(base_string).body(file).send().await?;
    Ok(())
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    let chat = Chat::from_env();
    let client = Client::new();

    let rt = Runtime::new().unwrap();
    let handle = rt.handle().clone();

    match args.photo {
        Some(path) => {
            log::info!("Sending photo...");
            handle
                .block_on(send_message(&client, &chat, &path))
                .unwrap();
            log::info!("Photo sent!");
        }
        None => {
            log::info!("Sending message...");
            handle
                .block_on(send_message(&client, &chat, &args.text))
                .unwrap();
            log::info!("Message sent!");
        }
    }
}
