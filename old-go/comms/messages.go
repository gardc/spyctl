package comms

import "encoding/json"

// MessageType represents the type of message being sent
type MessageType string

const (
	PingMessage MessageType = "Ping" // sent from slvate to master
	PongMessage MessageType = "Pong" // master responds to ping with pong

	SetSurveillanceStateType MessageType = "SetSurveillanceState"
	ScreenCaptureType        MessageType = "ScreenCapture"
	// Add more message types as needed
)

// Message is the base struct for all messages
type Message struct {
	Type MessageType     `json:"type"`
	Data json.RawMessage `json:"data"`
}

// SetSurveillanceStateMessage represents the message for setting surveillance state
type SetSurveillanceStateMessage struct {
	CaptureScreen     bool `json:"captureScreen"`
	CaptureScreenRate int  `json:"captureScreenRate"`
}

// ScreenCaptureMessage represents a screen capture message
type ScreenCaptureMessage struct {
	Image string `json:"image"` // Base64 encoded image data
}

// CreateMessage creates a new Message with the given type and data
func CreateMessage(messageType MessageType, data interface{}) ([]byte, error) {
	dataJSON, err := json.Marshal(data)
	if err != nil {
		return nil, err
	}

	message := Message{
		Type: messageType,
		Data: dataJSON,
	}

	return json.Marshal(message)
}

// ParseMessage parses a raw message and returns the message type and data
func ParseMessage(rawMessage []byte) (MessageType, json.RawMessage, error) {
	var message Message
	err := json.Unmarshal(rawMessage, &message)
	if err != nil {
		return "", nil, err
	}

	return message.Type, message.Data, nil
}
