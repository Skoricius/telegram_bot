use clap::Parser;
use reqwest::blocking::multipart::Form;
use reqwest::blocking::Client;
use reqwest::Error;
use std::{env, error};

static API_URL: &str = "https://api.telegram.org/";

/// Simple telegram communicator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Text to send
    #[clap(short, long, default_value = "Done!")]
    text: String,

    /// Document to send
    #[clap(short, long)]
    file: Option<String>,

    /// Picture to send
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

fn send_message(client: &Client, chat: &Chat, message: &str) -> Result<(), Error> {
    let base_string = chat.base_string("sendMessage");

    let _response = client
        .post(base_string)
        .query(&[("text", message)])
        .send()?;
    Ok(())
}

fn send_photo(client: &Client, chat: &Chat, path: &str) -> Result<(), Box<dyn error::Error>> {
    let base_string = chat.base_string("sendPhoto");
    let form = match Form::new().file("photo", path) {
        Ok(f) => f,
        Err(err) => {
            log::error!("Can not find the photo!");
            return Err(Box::new(err));
        }
    };

    let _response = client.post(base_string).multipart(form).send()?;
    Ok(())
}

fn send_file(client: &Client, chat: &Chat, path: &str) -> Result<(), Box<dyn error::Error>> {
    let base_string = chat.base_string("sendDocument");
    let form = match Form::new().file("document", path) {
        Ok(f) => f,
        Err(err) => {
            log::error!("Can not find the file!");
            return Err(Box::new(err));
        }
    };

    let _response = client.post(base_string).multipart(form).send()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();
    let args = Args::parse();
    let chat = Chat::from_env();
    let client = Client::new();

    if let Some(path) = args.photo {
        log::info!("Sending photo...");
        send_photo(&client, &chat, &path)?;
        log::info!("Photo sent!");
    } else if let Some(path) = args.file {
        log::info!("Sending file...");
        send_file(&client, &chat, &path)?;
        log::info!("File sent!");
    } else {
        log::info!("Sending message...");
        send_message(&client, &chat, &args.text)?;
        log::info!("Message sent!");
    }
    Ok(())
}
