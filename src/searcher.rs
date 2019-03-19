use std::path::{ Path, PathBuf };
// use std::io::{Error as IOError};
// use std::time::Duration;
use walkdir::{ WalkDir, DirEntry };

pub struct SearchEntry {
    path: Path,
    // age: Duration,
    // size: u64
}

impl SearchEntry {
    fn from_path(path: &PathBuf) -> SearchEntry {
        SearchEntry {
            path: path
        }
    }
}

pub struct SearchOptions {
    filter_entry: fn(&Path) -> bool,
    filter: fn(&DirEntry) -> bool
}

impl SearchOptions {
    fn default() -> SearchOptions {
        SearchOptions {
            filter_entry: |_| true,
            filter: |_| true
        }
    }
}

pub type SearchEnties = Vec<SearchEntry>;

pub fn search(root: &Path) -> Vec<SearchEntry> {
    search_with(root, SearchOptions::default());
}

pub fn search_with(root: &PathBuf, options: &SearchOptions) -> Vec<SearchEntry> {
    let paths: Vec<PathBuf> = WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| options.should_take)
        .filter_map(|r| r.ok())
        .filter(|e| options.filter)
        .map(|e| SearchEntry::from_path)
        .collect();
}
