use std::path::PathBuf;
use std::fs;
use std::env;

pub fn get_db_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let mut path = PathBuf::from(home);
    path.push("Documents");
    let _ = fs::create_dir_all(&path);
    path.push("data.db3");
    path
}