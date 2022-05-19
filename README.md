# Basic telegram message sending bot

## Building Rust version

1. Download and install [Rust](https://rustup.rs/)
2. Build with:
```
cargo run --release
```

The compiled binary is in `target/release/telegram`.

# Building Go version

1. Download and install Go
2. Build with:
```
go build telegram.go
```

# Using

1. Create a new bot using [Botfather](https://t.me/botfather) and get its token
2. Initialise the `TELEGRAM_TOKEN` and `TELEGRAM_CHAT_ID` environmental variables using the bots token and the chat id of the chat you want to send messages to.
```bash
export TELEGRAM_TOKEN=<token>
export TELEGRAM_CHAT_ID=<chat_id>
```
3. Use the built binary to send messages via:
```
telegram -t Hey!
```

The utility can also send files and photos. Check `telegram -h` for details.

You can also copy the compiled binary to `/usr/bin` and add the environmental variables to .bashrc to have it accessible as the terminal command.
