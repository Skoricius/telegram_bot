use reqwest;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let token =
        env::var("TELEGRAM_TOKEN").expect("Define the token with: export TELEGRAM_TOKEN=<token>");
    let chat_id = env::var("TELEGRAM_CHAT_ID")
        .expect("Define the chat id with: export TELEGRAM_CHAT_ID=<chat_id>");
    let message = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        String::from("Done!")
    };

    let request_string = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
        token, chat_id, message
    );

    let _resp = reqwest::blocking::get(request_string);
    // println!("{:#?}", resp);
}
