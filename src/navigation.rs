use std::path::{PathBuf, Path};

pub struct Navigator {
    current_path: PathBuf,
    history: Vec<PathBuf>,
    history_index: usize,
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            current_path: PathBuf::from("/"),
            history: vec![PathBuf::from("/")],
            history_index: 0,
        }
    }

    pub fn navigate_to(&mut self, path: &Path) {
        self.current_path = path.to_path_buf();
        self.history.push(self.current_path.clone());
        self.history_index = self.history.len() - 1;
    }

    pub fn current_path(&self) -> &Path {
        &self.current_path
    }
}

