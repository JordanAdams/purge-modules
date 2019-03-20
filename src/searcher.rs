use std::path::{ Path, PathBuf };
// use std::io::{Error as IOError};
// use std::time::Duration;
use walkdir::{ WalkDir, DirEntry };

#[derive(Debug)]
pub struct SearchEntry {
    path: PathBuf,
    // age: Duration,
    // size: u64
}

impl SearchEntry {
    fn from_path(path: &PathBuf) -> SearchEntry {
        SearchEntry {
            path: path.to_path_buf()
        }
    }

    pub fn path(&self) -> PathBuf {
        self.path.to_path_buf()
    }

    pub fn is_dir(&self) -> bool {
        self.path().is_dir() == true
    }
}

pub struct SearchOptions {
    pub filter_entry: fn(&DirEntry) -> bool,
    pub filter: fn(&SearchEntry) -> bool
}

impl SearchOptions {
    fn default() -> SearchOptions {
        SearchOptions {
            filter_entry: |_| true,
            filter: |_| true
        }
    }
}

pub type SearchEntries = Vec<SearchEntry>;

pub fn search(root: &Path) -> Vec<SearchEntry> {
    search_with(root, &SearchOptions::default())
}

pub fn search_with(root: &Path, options: &SearchOptions) -> SearchEntries {
    WalkDir::new(root)
        .into_iter()
        .filter_entry(options.filter_entry)
        .filter_map(|r| r.ok())
        .map(|e| SearchEntry::from_path(&e.path().to_path_buf()))
        .filter(options.filter)
        .collect()
}
