use crate::desktop_files::{application_file::ApplicationFile, run_application::run_application};

pub enum Action {
    Application(ApplicationFile),
    ShellCommand(String)
}

impl Action {
    fn run(&self) -> Result<(), String> {
        match self {
            Action::Application(a) => run_application(a).map(|_| ()),
            Action::ShellCommand(_) => todo!(),
        }
    }
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Action::Application(a) => a.app_name.clone(),
            Action::ShellCommand(c) => "$ ".to_string() + c,
        }
    }
}