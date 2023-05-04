use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox, Settings};

use std::sync::Arc;

use crate::actions::{action::Action, action_database::ActionDatabase};


pub fn open_window(actions: ActionDatabase) -> Result<(), iced::Error> {
    SearchingWindow::run(Settings::default())
}


fn get_action_results(actions: &ActionDatabase, query: String) -> Vec<Arc<Action>> {
    actions
        .get_action_results(&query)
        .iter()
        .map(|x| x.clone())
        .collect()
}

#[derive(Clone, Default)]
pub struct SearchingWindow {
    pub search_query: String,
    pub selected_index: usize,
    pub results: Arc<Vec<Arc<Action>>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ClickOption,
    SelectNext,
    SelectPrevious,
    Search,
    ScrollUp,
    ScrollDown,
    Resize
}

impl Sandbox for SearchingWindow {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Searchy".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        println!("{:?}", message);
    }

    fn view(&self) -> Element<Self::Message> {
        column!(
            text("hello world! :)").size(50),
        )
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

impl SearchingWindow {
    pub fn new(results: Vec<Arc<Action>>) -> SearchingWindow {
        SearchingWindow {
            search_query: "".into(),
            selected_index: results.len() - 1,
            results: Arc::new(results),
        }
    }
    pub fn get_selected(&self) -> Option<Arc<Action>> {
        return self.results.get(self.selected_index).cloned();
    }
    pub fn run_selected(&self) -> () {
        let Some(to_run) = &self.get_selected() else { return };

        match to_run.run() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error launching: {}", e)
            }
        }
    }
}
