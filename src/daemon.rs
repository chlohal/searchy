#![feature(option_result_contains)]

use actions::{action_database::ActionDatabase, action::Action};
use path_executables::path_executable_search::path_executables;
use desktop_files::desktop_file_search::application_files;
use interface::window::open_window;

mod interface;
mod config;
mod desktop_files;
mod actions;
mod path_executables;

fn main() {
    let mut actions = ActionDatabase::new();

    for app in application_files() {
        actions.add(Action::Application(app));
    }

    for executable in path_executables().take(300) {
        actions.add(Action::ShellCommand(executable));
    }

    let window = open_window(actions);
}