#![feature(option_result_contains)]

use std::{io::Read, os::unix::net::UnixStream, process::exit, sync::Arc};

use actions::{action::Action, action_database::ActionDatabase};
use desktop_files::desktop_file_search::application_files;
use interface::window::open_window;
use ipc_communication::{server_side::listen_socket, message::IpcMessage};
use path_executables::path_executable_search::path_executables;

mod actions;
mod desktop_files;
mod interface;
mod ipc_communication;
mod path_executables;

fn main() {
    let mut actions = ActionDatabase::new();

    for app in application_files() {
        actions.add(Action::Application(app));
    }

    for executable in path_executables() {
        actions.add(Action::ShellCommand(executable));
    }

    let actions_arc = Arc::new(actions);

    match listen_socket() {
        Ok(listener) => loop {
            if let Ok((unix_stream, _socket_address)) = listener.accept() {
                handle_stream(unix_stream, &actions_arc);
            }
        },
        Err(err) => eprintln!("{:?}", err),
    }
}

fn handle_stream(mut unix_stream: UnixStream, actions: &Arc<ActionDatabase>) {
    let mut buf = Vec::<u8>::new();
    match unix_stream.read_to_end(&mut buf) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err.to_string());
            return;
        }
    };

    let message = match String::from_utf8(buf) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err.to_string());
            return;
        }
    };

    let ipc_message: IpcMessage = match message.try_into() {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err.to_string());
            return;
        },
    };

    match ipc_message {
        IpcMessage::OpenWindow => {
            match open_window(actions.clone()) {
                Ok(_) => {}
                Err(err) => eprintln!("{}", err.to_string()),
            };
            exit(0);
        }
    }
}
