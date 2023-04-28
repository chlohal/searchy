use std::{fmt::Display, path::PathBuf};

use crate::desktop_files::{application_file::ApplicationFile, run_application::run_application};

pub enum Action {
    Application(ApplicationFile),
    ShellCommand(PathBuf),
}

impl Action {
    pub fn run(&self) -> Result<(), String> {
        match self {
            Action::Application(a) => run_application(a).map(|_| ()),
            Action::ShellCommand(_) => todo!(),
        }
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Application(l0), Self::Application(r0)) => l0 == r0,
            (Self::ShellCommand(l0), Self::ShellCommand(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Action::Application(a) => "  ".to_string() + &a.app_name,
            Action::ShellCommand(cmd) => {
                "$ ".to_string()
                    + &cmd.file_name().map_or_else(
                        || cmd.to_string_lossy(),
                        |basename| basename.to_string_lossy(),
                    ) + &cmd.to_string_lossy()
            }
        }
    }
}
