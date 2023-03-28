use std::{env::var_os, path::PathBuf};

pub const PROGRAM_NAME: &str = "searchy";

pub fn index_directory() -> Option<PathBuf> {
    let mut path: PathBuf = var_os("XDG_STATE_HOME")
    .or(var_os("XDG_CACHE_HOME"))
    .and_then(|x| x.try_into().ok())
    .or_else(|| {
        let mut path: PathBuf = var_os("HOME")?.try_into().ok()?;
        path.push(".local");
        path.push("share");
        Some(path)
    })?;

    path.push(PROGRAM_NAME);
    path.push("index");

    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => Some(path),
        Err(_) => None,
    }
}