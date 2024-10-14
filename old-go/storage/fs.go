package storage

import (
	"fmt"
	"os"
	"time"
)

// Stores a screenshot to the ./screenshots/ folder
func StoreScreenshot(screenshot []byte) (string, error) {
	// Create the screenshots directory if it doesn't exist
	err := os.MkdirAll("./screenshots", 0755)
	if err != nil {
		return "", fmt.Errorf("failed to create screenshots directory: %w", err)
	}

	// Generate a filename with the current timestamp
	filename := fmt.Sprintf("./screenshots/screenshot_%s.jpg", time.Now().Format("20060102_150405"))

	// Write the screenshot bytes to the file
	err = os.WriteFile(filename, screenshot, 0644)
	if err != nil {
		return "", fmt.Errorf("failed to write screenshot file: %w", err)
	}

	return filename, nil
}
