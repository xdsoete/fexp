use std::{fs, path::PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub hide_hidden_files: bool,
    pub sorting_stratgy: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            hide_hidden_files: true,
            sorting_stratgy: "alpha".to_string()
        }
    }
}

impl Config {
    fn config_path() -> PathBuf {
        let mut path = dirs_2::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("fexp");
        fs::create_dir_all(&path).ok();
        path.push("config.toml");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            toml::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}
