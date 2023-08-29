use iced::{widget::{Scrollable, scrollable}, Command};
use messages::Message;

pub struct ReplInput {
    input: String
}
impl ReplInput {
    pub fn update(&self, message: messages::SearchResultMessage) -> Command<Message> {
        match message {
            messages::SearchResultMessage::SelectNext => todo!(),
            messages::SearchResultMessage::SelectPrevious => todo!(),
            messages::SearchResultMessage::Scroll(_) => todo!(),
            messages::SearchResultMessage::Search(_) => todo!(),
            messages::SearchResultMessage::ClickOption(_) => todo!(),
            messages::SearchResultMessage::LaunchSelected => todo!(),
        }
    }
}

impl Default for ReplInput {
    fn default() -> Self {
        Self { input: Default::default() }
    }
}

pub fn repl_view(repl: &ReplInput) -> Scrollable<'static, Message> {
    scrollable(iced::widget::text(&repl.input))
}