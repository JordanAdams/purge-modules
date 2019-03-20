extern crate dirs;
extern crate walkdir;

mod searcher;

use std::fs::{File};
use std::io::Error as IOError;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::{ DirEntry };
use searcher::{ search_with, SearchOptions, SearchEntry };
use dirs::home_dir;

const ONE_MONTH_IN_SECONDS: u64 = 2629746;

fn root_dir() -> PathBuf {
    let home = home_dir().expect("Unable to detect home directory");

    Path::join(home.as_path(), Path::new("code"))
}

fn is_within_node_modules(entry: &DirEntry) -> bool {
    entry
        .path()
        .ancestors()
        .skip(1)
        .any(|a| a.ends_with("node_modules"))
}

fn should_delete(entry: &SearchEntry) -> Result<bool, IOError> {
    File::open(entry.path())
        .and_then(|f| f.metadata())
        .and_then(|m| m.modified())
        .and_then(|m| Ok(SystemTime::now().duration_since(m).unwrap()))
        .and_then(|d| Ok(d.as_secs() > ONE_MONTH_IN_SECONDS))
}

fn main() {
    let options = SearchOptions {
        filter_entry: |e| !is_within_node_modules(e),
        filter: |e| {
            e.is_dir()
                && e.path().ends_with("node_modules")
                && should_delete(e).unwrap_or(false)
        }
    };

    for entry in search_with(&root_dir(), &options) {
        println!("{}", entry.path().display());
        // remove_dir(path).expect(format!("Failed to delete {}", path.display()));
    }
}
