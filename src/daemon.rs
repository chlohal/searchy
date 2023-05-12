#![feature(option_result_contains)]

use std::{io::Read, os::unix::net::UnixStream, process::exit, sync::Arc, env::args};

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

    open_window(actions_arc.clone());
}