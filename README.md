# Basic telegram message sending bot

## Building

1. Download and install [Rust](https://rustup.rs/)
2. Build with:
```
cargo run --release
```

# Using
1. Create a new bot using [Botfather](https://t.me/botfather) and get its token
2. Initialise the `TELEGRAM_TOKEN` and `TELEGRAM_CHAT_ID` environmental variables using the bots token and the chat id of the chat you want to send messages to.
3. Use the built utility in `target/release/telegram` to send messages via:
```
target/release/telegram Hey!
```
