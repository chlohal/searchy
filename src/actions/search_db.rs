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

pub fn score_shell_command(cmd: &String, query: &String, matcher: &SkimMatcherV2) -> i64 {
    matcher.fuzzy_match(query.as_str(), query.as_str()).unwrap_or_default()
}