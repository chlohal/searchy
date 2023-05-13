use std::path::Path;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use crate::varieties::desktop_files::application_file::ApplicationFile;

use super::action::Action;

pub fn score_action(action: &Action, query: &str, matcher: &SkimMatcherV2) -> i64 {
    match action {
        Action::Application(app) => score_application(app, query, matcher),
        Action::ShellCommand(cmd) => score_shell_command(cmd, query, matcher),
    }
}

pub fn score_application(app: &ApplicationFile, query: &str, matcher: &SkimMatcherV2) -> i64 {
    matcher.fuzzy_match(&app.app_name, query).unwrap_or_default()
}

pub fn score_shell_command(cmd: &Path, query: &str, matcher: &SkimMatcherV2) -> i64 {
    matcher.fuzzy_match(&cmd.as_os_str().to_string_lossy(), query).unwrap_or_default()
}