use std::fs;
use std::path::Path;
use std::process::Command;

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
