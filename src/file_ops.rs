use std::fs;
use std::path::Path;
use std::process::Command;

pub enum FileType {
    Rust,
    Python,
    Text,
    Folder,
    Unknown,
}

pub fn list_directory_contents(path: &Path) -> Vec<String> {
    let mut entries = Vec::new();
    if let Ok(dir_entries) = fs::read_dir(path) {
        for entry in dir_entries {
            if let Ok(entry) = entry {
                entries.push(entry.file_name().into_string().unwrap());
            }
        }
    }
    entries
}

pub fn open_file(path: &Path) {
    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
        .arg(path)
        .spawn()
        .expect("Failed to open file");

    #[cfg(target_os = "macos")]
    Command::new("open")
        .arg(path)
        .spawn()
        .expect("Failed to open file");

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args("/C", "start", path)
        .spawn()
        .expect("Failed to open file");
}

pub fn get_file_type(path: &Path) -> FileType {
    if path.is_dir(){
        FileType::Folder
    } else if let Some(ext) = path.extension() {
        match ext.to_str() {
            Some("rs") => FileType::Rust,
            Some("py") => FileType::Python,
            Some("txt") => FileType::Text,
            _ => FileType::Unknown,
        }
    } else {
        FileType::Unknown
    }
}
