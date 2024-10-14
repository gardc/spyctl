package comms

import (
	"encoding/base64"
	"log"
	"net/url"

	"github.com/gorilla/websocket"
)

const encryptedMasterIP string = "bG9jYWxob3N0OjY5Njk=" // localhost:6969

func GetMasterIP() string {
	data, err := base64.StdEncoding.DecodeString(encryptedMasterIP)
	if err != nil {
		log.Fatalf("Failed to decode master IP: %v", err)
	}
	return string(data)
}

// This is the connection manager/service for the slave. The slave should only have one connection to the master,
// and the SlaveConn should manage the connection and make sure it's reopened if closed. It should also buffer
// messages and send them to the master when the connection is open.

type SlaveConn struct {
	conn *websocket.Conn

	Disconnected chan struct{}
	Send         chan Message
	Received     chan Message
}

func SpawnNewConnection() *SlaveConn {
	sc := &SlaveConn{
		Send:         make(chan Message, 64),
		Received:     make(chan Message, 64),
		Disconnected: make(chan struct{}),
	}
	go sc.startAndManageConnection()
	return sc
}

func (sc *SlaveConn) startAndManageConnection() {
	// Create the URL connection to master
	addr := GetMasterIP()
	u := url.URL{Scheme: "ws", Host: addr, Path: "/echo"}
	log.Printf("connecting to %s", u.String())

	var err error
	sc.conn, _, err = websocket.DefaultDialer.Dial(u.String(), nil)
	if err != nil {
		log.Fatal("dial:", err)
	}
	defer sc.conn.Close()

	// connected to master, start reading messages
	go sc.processRecvMessages()
	go sc.processSendMessages()
}

// processRecvMessages reads messages, parses them and sends them through the received channel until the connection is closed.
func (sc *SlaveConn) processRecvMessages() {
	for {
		select {
		case <-sc.Disconnected:
			return
		default:
			_, message, err := sc.conn.ReadMessage()
			if err != nil {
				log.Println("read:", err)
				return
			}
			// parse the message
			msgType, rawMsg, err := ParseMessage(message)
			if err != nil {
				log.Fatal(err)
			}
			msg := Message{
				Type: msgType,
				Data: rawMsg,
			}
			sc.Received <- msg
		}
	}
}

func (sc *SlaveConn) processSendMessages() {
	for {
		select {
		case <-sc.Disconnected:
			return
		case msg := <-sc.Send:
			encodedMsg, err := CreateMessage(msg.Type, msg.Data)
			if err != nil {
				log.Fatal(err)
			}
			sc.conn.WriteMessage(websocket.TextMessage, encodedMsg)
		}
	}
}
