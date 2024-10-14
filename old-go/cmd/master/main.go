package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool { return true }, // Allow all origins (not recommended for production)
}

func MasterWSHandler(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Printf("Failed to upgrade connection: %v", err)
		return
	}
	defer conn.Close()

	log.Println("Slave connected")
	for {
		_, msg, err := conn.ReadMessage()
		if err != nil {
			log.Println("Read error:", err)
			break
		}
		log.Printf("Received from slave: %s", msg)

		// Respond to the slave
		response := fmt.Sprintf("Echo: %s", msg)
		if err := conn.WriteMessage(websocket.TextMessage, []byte(response)); err != nil {
			log.Println("Write error:", err)
			break
		}
	}
}

func main() {
	http.HandleFunc("/", MasterWSHandler)
	log.Println("Master server listening on :6969")
	log.Fatal(http.ListenAndServe(":6969", nil))
}
