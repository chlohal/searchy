use std::path::PathBuf;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use crate::desktop_files::application_file::ApplicationFile;

use super::action::Action;

pub fn score_action(action: &Action, query: &String, matcher: &SkimMatcherV2) -> i64 {
    match action {
        Action::Application(app) => score_application(app, query, matcher),
        Action::ShellCommand(cmd) => score_shell_command(cmd, query, matcher),
    }
}

pub fn score_application(app: &ApplicationFile, query: &String, matcher: &SkimMatcherV2) -> i64 {
    matcher.fuzzy_match(&app.app_name, query.as_str()).unwrap_or_default()
}

pub fn score_shell_command(cmd: &PathBuf, query: &String, matcher: &SkimMatcherV2) -> i64 {
    matcher.fuzzy_match(&cmd.as_os_str().to_string_lossy(), query.as_str()).unwrap_or_default()
}