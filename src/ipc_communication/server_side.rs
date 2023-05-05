use std::os::unix::net::UnixListener;

use super::socket_file::get_socket_file_address;

pub fn listen_socket() -> Result<UnixListener, std::io::Error> {
    let socket_path = get_socket_file_address();

    if std::fs::metadata(&socket_path).is_ok() {
        std::fs::remove_file(&socket_path)?;
    }

    UnixListener::bind(socket_path)
}