use actions::{action::Action, action_database::ActionDatabase};
use varieties::{
    desktop_files::desktop_file_search::application_files,
    path_executables::path_executable_search::path_executables,
};

pub mod actions;
pub mod varieties;

pub fn actions() -> impl Iterator<Item = Action> {
    let path_actions = path_executables().map(|x| Action::ShellCommand(x));

    let app_actions = application_files().map(|x| Action::Application(Box::new(x)));

    path_actions.chain(app_actions)
}

pub fn load_db() -> ActionDatabase {
    let mut db = ActionDatabase::new();

    for a in actions() {
        db.add(a);
    }

    db
}