use ipc_communication::{client_side::send_socket, message::IpcMessage};
use std::env::args;

fn main() {
    let mut has_sent = 0;
    for arg in args().skip(1) {
        match send_socket(arg) {
            Ok(_) => {
                has_sent += 1;
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }

    if has_sent == 0 {
        eprintln!(
            r#"USAGE: searchy [messages...]

    message options:
{}"#,
            IpcMessage::representations()
                .iter()
                .map(|x| format!("    - {}\n", x))
                .collect::<String>()
        );
    }
}
