package surveillance

import (
	"bytes"
	"image"
	"image/jpeg"

	"github.com/kbinani/screenshot"
)

var jpegOptions = &jpeg.Options{Quality: 70} // Set quality from 1 to 100

func Screenshot() ([][]byte, error) {
	imgs, err := getRawScreenshot()
	if err != nil {
		return nil, err
	}

	bufs := make([][]byte, len(imgs))
	for i, img := range imgs {
		buf := new(bytes.Buffer)
		if err := jpeg.Encode(buf, img, jpegOptions); err != nil {
			return nil, err
		}
		bufs[i] = buf.Bytes()
	}
	return bufs, nil
}

func getRawScreenshot() ([]*image.RGBA, error) {
	n := screenshot.NumActiveDisplays()

	imgs := make([]*image.RGBA, n)

	for i := 0; i < n; i++ {
		bounds := screenshot.GetDisplayBounds(i)

		img, err := screenshot.CaptureRect(bounds)
		if err != nil {
			return nil, err
		}

		imgs[i] = img
	}

	return imgs, nil
}
