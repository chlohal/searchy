use std::{os::unix::net::UnixStream, io::Write};

use super::socket_file::get_socket_file_address;

pub fn send_socket(message: String) -> Result<(), std::io::Error> {
    let socket_path = get_socket_file_address();

    let mut unix_stream = UnixStream::connect(socket_path)?;

    unix_stream.write_all(message.as_bytes())
}