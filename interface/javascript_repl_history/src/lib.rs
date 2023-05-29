use iced::widget::{Scrollable, scrollable};
use messages::Message;

pub struct ReplInput {
    input: String
}

impl Default for ReplInput {
    fn default() -> Self {
        Self { input: Default::default() }
    }
}

pub fn repl_view(repl: &ReplInput) -> Scrollable<'static, Message> {
    scrollable(iced::widget::text("text"))
}