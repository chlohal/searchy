use actions::{action_database::ActionDatabase, action::Action};
use desktop_files::desktop_file_search::application_files;
use interface::window::open_window;

mod interface;
mod config;
mod desktop_files;
mod actions;

fn main() {
    let mut actions = ActionDatabase::new();

    for app in application_files() {
        actions.add(Action::Application(app))
    }

    open_window(actions);
}