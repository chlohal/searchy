extern crate skim;

use std::thread;
use skim::prelude::*;

use crate::{
    application_file::{parse_application_file, ApplicationFile, ApplicationType},
    ApplicationFileSearch,
};

impl SkimItem for ApplicationFile {
    fn text(&self) -> Cow<str> {
        let mut string: String = String::new();

        string.push_str(&self.app_name);
        string.push_str("\t");
        string.push_str(self.app_comment.as_ref().unwrap_or(&"".into()));
        string.push_str(
            self.app_keywords
                .as_ref()
                .map(|x| x.join(", "))
                .unwrap_or("".into())
                .as_str(),
        );

        return Cow::from(string);
    }

    fn display<'a>(&'a self, context: DisplayContext<'a>) -> AnsiString<'a> {
        let mut string: String = String::new();

        string.push_str(&self.app_name);
        string.push_str("\t");
        string.push_str(self.app_comment.as_ref().unwrap_or(&"".into()));

        return AnsiString::parse(string.as_str());
    }
}

pub fn search_em_up(apps: ApplicationFileSearch) -> Option<ApplicationFile> {
    let options = SkimOptionsBuilder::default().build().unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    thread::spawn(|| {
        for app_file in apps {
            if let Ok(app) = parse_application_file(app_file) {

                if app.app_keywords.is_some() && app.app_categories.is_some() {
                    match app.app_type {
                        ApplicationType::Application => tx.send(Arc::new(app)).unwrap(),
                        _ => ()
                    }
                }
            }
        }
        drop(tx);
    });

    let skim_search = Skim::run_with(&options, Some(rx))?;

    if skim_search.is_abort {
        return None;
    }

    let selected_items = skim_search.selected_items;

    let first_item = selected_items.iter().next().map(|selected_item| {
        (**selected_item)
            .as_any()
            .downcast_ref::<ApplicationFile>()
            .unwrap()
    })?;

    return Some(first_item.to_owned());
}
