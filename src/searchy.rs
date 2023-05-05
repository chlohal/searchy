mod ipc_communication;

use ipc_communication::client_side::send_socket;
use std::env::args;

fn main() {
    let mut has_sent = 0;
    for arg in args().skip(1) {
        match send_socket(arg) {
            Ok(_) => {
                has_sent += 1;
            }
            Err(err) => {
                eprintln!("{}", err.to_string());
            }
        }
    }

    if has_sent == 0 {
        eprintln!(
            r#"USAGE: searchy [messages...]
        
    message options:
        - open-window"#
        );
    }
}
