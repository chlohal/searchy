#![feature(option_result_contains)]

use std::{sync::Arc};

use actions::{action::Action, action_database::ActionDatabase};
use desktop_files::desktop_file_search::application_files;
use interface::window::open_window;

use path_executables::path_executable_search::path_executables;

mod actions;
mod desktop_files;
mod interface;
mod ipc_communication;
mod path_executables;

fn main() {
    let mut actions = ActionDatabase::new();

    for app in application_files() {
        actions.add(Action::Application(Box::new(app)));
    }

    for executable in path_executables() {
        actions.add(Action::ShellCommand(executable));
    }

    let actions_arc = Arc::new(actions);

    open_window(actions_arc).unwrap();
}