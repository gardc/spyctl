use std::fs;

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
