package main

import (
	"fmt"
	"net/http"
	"os"
	"strings"
)

func main() {
	token := os.Getenv("TELEGRAM_TOKEN")
	if token == "" {
		fmt.Println("Define the token with: export TELEGRAM_TOKEN=<token>")
		return
	}
	chat_id := os.Getenv("TELEGRAM_CHAT_ID")
	if chat_id == "" {
		fmt.Println("Define the chat id with: export TELEGRAM_CHAT_ID=<chat_id>")
		return
	}
	var message string
	if len(os.Args) > 1 {
		message = strings.Join(os.Args[1:], " ")
	} else {
		message = "Done!"
	}

	api_request := fmt.Sprintf("https://api.telegram.org/bot%s/sendMessage?chat_id=%s&text=%s",
		token, chat_id, message)

	_, err := http.Get(api_request)
	if err != nil {
		fmt.Println(err)
	}
}
