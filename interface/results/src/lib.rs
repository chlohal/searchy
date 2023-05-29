use std::sync::Arc;

use actions::{actions::Action, ActionDatabase};
use iced::{
    widget::{scrollable, text_input, Scrollable},
    Command,
};
use interface_scrolling::{results_scrollbox, SCROLLABLE_ID};
use interface_searchbox::SEARCHBOX_ID;
use javascript_repl_history::repl_view;
use messages::{Message, SearchResultMessage};

pub struct ActionsSearch {
    pub selected: Option<Arc<Action>>,
    pub actions: Arc<ActionDatabase>,
    pub results: Vec<Arc<Action>>,
    pub scroll_top: f32,
}

impl ActionsSearch {
    pub fn update(&mut self, message: SearchResultMessage) -> iced::Command<Message> {
        match message {
            SearchResultMessage::Search(query) => {
                self.results = self.actions.get_action_results(&query).to_vec();
                self.selected = self.results.first().cloned();

                self.scroll_top = 0.0;
                scrollable::snap_to(SCROLLABLE_ID.clone(), scrollable::RelativeOffset::START)
            }
            SearchResultMessage::ClickOption(action) => {
                self.selected = Some(action);

                //refocus searchbox
                text_input::focus(SEARCHBOX_ID.clone())
            }
            SearchResultMessage::Scroll(y) => {
                self.scroll_top = y;
                Command::none()
            }
            SearchResultMessage::LaunchSelected => {
                self.run_selected();
                Command::perform((|| async { Message::HideWindow })(), |x| x)
            }
            SearchResultMessage::SelectNext => {
                let Some(selected) = &self.selected else { return Command::none() };

                let Some(index) = self.results.iter().position(|x| x == selected) else { return Command::none() };

                let Some(new_selected) = self.results.get(index + 1) else { return Command::none() };

                self.selected = Some(new_selected.clone());

                //scroll_to_view(index + 1, self.results.len())
                Command::none()
            }
            SearchResultMessage::SelectPrevious => {
                let Some(selected) = &self.selected else { return Command::none() };

                let Some(index) = self.results.iter().position(|x| x == selected) else { return Command::none() };

                if index == 0 {
                    return Command::none();
                }

                let Some(new_selected) = self.results.get(index - 1) else { return Command::none() };

                self.selected = Some(new_selected.clone());

                //scroll_to_view(index - 1, self.results.len())
                Command::none()
            }
        }
    }

    pub fn run_selected(&self) {
        let Some(to_run) = &self.selected else { return };

        match to_run.run() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error launching: {}", e)
            }
        }
    }
}
pub enum SearchType {
    ApplicationLaunch(ActionsSearch),
    ActionSubmenu(ActionsSearch),
    JavascriptRepl,
}

impl SearchType {
    pub fn update(&mut self, message: SearchResultMessage) -> iced::Command<Message> {
        match self {
            SearchType::ActionSubmenu(actions) | SearchType::ApplicationLaunch(actions) => {
                actions.update(message)
            }
            SearchType::JavascriptRepl => todo!(),
        }
    }
}

pub fn results_view(search: &SearchType) -> Scrollable<'static, Message> {
    match search {
        SearchType::ApplicationLaunch(search) => {
            results_scrollbox(&search.results, search.scroll_top, &search.selected)
        }
        SearchType::ActionSubmenu(_) => todo!(),
        SearchType::JavascriptRepl => repl_view(),
    }
}
