use image::codecs::jpeg::JpegEncoder;
use serde::{Deserialize, Serialize};
use xcap::Monitor;

use crate::common::util::{ensure_directory_exists, normalize_filename};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screenshot {
    pub image: Vec<u8>,
    pub name: String,
}

pub fn capture_screens() -> Vec<Screenshot> {
    let monitors = Monitor::all().unwrap();
    let mut screenshots: Vec<Screenshot> = vec![];

    for monitor in monitors {
        let image = monitor.capture_image().unwrap();

        let mut jpeg_data = Vec::new();
        let mut jpeg_encoder = JpegEncoder::new_with_quality(&mut jpeg_data, 33);
        jpeg_encoder.encode_image(&image).unwrap();

        let name = format!(
            "{}-{}",
            monitor.name().to_string(),
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string()
        );

        screenshots.push(Screenshot {
            image: jpeg_data,
            name,
        });
    }

    screenshots
}

pub fn save_sceenshots(screenshots: Vec<Screenshot>, ip: String) {
    ensure_directory_exists("slave-screenshots").unwrap();

    for screenshot in screenshots {
        // Save as JPEG with 50% quality
        std::fs::write(
            format!(
                "slave-screenshots/{}__{}_monitor-{}.jpg",
                ip,
                "screenshots",
                normalize_filename(&screenshot.name)
            ),
            screenshot.image,
        )
        .unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_capture_screenshot() {
        let screens = capture_screens();
        assert!(screens.len() > 0);

        save_sceenshots(screens, "test".to_string());
    }
}
