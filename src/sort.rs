use std::path::PathBuf;

pub trait Sorter {
    fn sort(&self, files: Vec<PathBuf>) -> Vec<PathBuf>;
}

pub struct Alphabetical;
pub struct AlphabeticalDirectoriesFirst;

impl Sorter for Alphabetical {
    fn sort(&self, mut files: Vec<PathBuf>) -> Vec<PathBuf> {
        files.sort();
        files
    }
}

impl Sorter for AlphabeticalDirectoriesFirst {
    fn sort(&self, mut files: Vec<PathBuf>) -> Vec<PathBuf> {
        files.sort_by(|a,b| {
            let a_dir = a.is_dir();
            let b_dir = b.is_dir();

            if a_dir == b_dir {
                a.to_string_lossy().cmp(&b.to_string_lossy())
            } else if a_dir {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        files
    }
}
