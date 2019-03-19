extern crate dirs;
extern crate walkdir;

mod searcher;

use std::fs::{File};
use std::io::Error as IOError;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use searcher::search;
use dirs::home_dir;

const ONE_MONTH_IN_SECONDS: u64 = 2629746;

fn root_dir() -> PathBuf {
    let home = home_dir().expect("Unable to detect home directory");

    Path::join(home.as_path(), Path::new("code"))
}

fn is_within_node_modules(p: &Path) -> bool {
    p.ancestors().skip(1).any(|a| a.ends_with("node_modules"))
}

fn should_delete(p: &Path) -> Result<bool, IOError> {
    File::open(p)
        .and_then(|f| f.metadata())
        .and_then(|m| m.modified())
        .and_then(|m| Ok(SystemTime::now().duration_since(m).unwrap()))
        .and_then(|d| Ok(d.as_secs() > ONE_MONTH_IN_SECONDS))
}

fn main() {
    // let paths: Vec<PathBuf> = WalkDir::new(root_dir())
    //     .into_iter()
    //     .filter_entry(|e| !is_within_node_modules(e.path()))
    //     .filter_map(|r| r.ok())
    //     .filter(|e| e.path().ends_with("node_modules"))
    //     .map(|e| e.path().to_owned())
    //     .collect();

    // for path in paths {
    //     match should_delete(&path) {
    //         Ok(true) => {
    //             println!("Deleting {}", path.display());
    //             // remove_dir(path).expect(format!("Failed to delete {}", path.display()));
    //         }
    //         _ => (),
    //     }
    // }
}
