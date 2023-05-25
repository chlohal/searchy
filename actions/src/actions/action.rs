use std::hash::Hash;
use std::path::PathBuf;

use crate::varieties::desktop_files::{
    application_file::ApplicationFile, run_application::run_application,
};

use crate::varieties::path_executables::run_shell_command::run_shell_command;

#[derive(Debug)]
pub enum Action {
    Application(Box<ApplicationFile>),
    ShellCommand(PathBuf),
}

impl Hash for Action {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Action::Application(a) => a.hash(state),
            Action::ShellCommand(p) => p.hash(state),
        }
    }
}

impl Action {
    pub fn run(&self) -> Result<(), String> {
        match self {
            Action::Application(a) => run_application(a).map(|_| ()),
            Action::ShellCommand(cmd) => run_shell_command(cmd).map(|_| ()),
        }
    }

    pub fn search_queriable(&self) -> String {
        match self {
            Action::Application(a) => a.app_name.clone(),
            Action::ShellCommand(cmd) => format!(
                "{} ({})",
                try_basename(cmd).replace("-", " "),
                cmd.to_string_lossy()
            ),
        }
    }
    pub fn primary_name(&self) -> String {
        match self {
            Action::Application(a) => a.app_name.clone(),
            Action::ShellCommand(p) => try_basename(p),
        }
    }
}

fn try_basename(cmd: &PathBuf) -> String {
    cmd.file_name()
                    .map_or_else(
                        || cmd.to_string_lossy(),
                        |basename| basename.to_string_lossy()
                    ).to_string()
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
                format!(
                    "$ {} ({})",
                    &cmd.file_name().map_or_else(
                        || cmd.to_string_lossy(),
                        |basename| basename.to_string_lossy()
                    ),
                    &cmd.to_string_lossy()
                )
            }
        }
    }
}
