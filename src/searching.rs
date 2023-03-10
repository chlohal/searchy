extern crate skim;

use skim::prelude::*;

use crate::{
    application_file::{parse_application_file, ApplicationFile},
    ApplicationFileSearch,
};

impl SkimItem for ApplicationFile {
    fn text(&self) -> Cow<str> {
        let mut string: String = String::new();

        string.push_str(&self.app_name);
        string.push_str("\t");
        string.push_str(self.app_comment.as_ref().unwrap_or(&"".into()));
        string.push_str(self.app_keywords.as_ref().map(|x| x.join(", ")).unwrap_or("".into()).as_str());

        return Cow::from(string);
    }

    fn display<'a>(&'a self, context: DisplayContext<'a>) -> AnsiString<'a> {
        let mut string: String = String::new();

        string.push_str(&self.app_name);
        string.push_str("\t");
        string.push_str(self.app_comment.as_ref().unwrap_or(&"".into()));
        AnsiString::new_empty().
        return AnsiString::parse(string.as_str())
    }
}

pub fn search_em_up(apps: ApplicationFileSearch) -> Option<ApplicationFile> {
    let options = SkimOptionsBuilder::default().build().unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for app_file in apps {
        if let Ok(app) = parse_application_file(app_file) {
            tx.send(Arc::new(app)).unwrap();
        }
    }

    drop(tx);

    let selected_items = Skim::run_with(&options, Some(rx))?.selected_items;

    
    let first_item = selected_items.iter()
        .next()
        .map(|selected_item| {
            (**selected_item)
                .as_any()
                .downcast_ref::<ApplicationFile>()
                .unwrap()
        })?;

    return Some(first_item.to_owned());
}
