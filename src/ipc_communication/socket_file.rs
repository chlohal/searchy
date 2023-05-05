use std::{env, path::PathBuf};

pub fn get_socket_file_address() -> PathBuf {
    let runtime_dir = env::var("XDG_RUNTIME_DIR").unwrap_or("~/.cache".to_string());

    PathBuf::from(runtime_dir).join("searchy-communicate.sock")
}