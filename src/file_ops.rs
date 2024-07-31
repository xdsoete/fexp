use std::fs;
use std::path::Path;

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

