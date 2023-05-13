use iced::{widget::{text_input, TextInput}, Length};
use messages::Message;
use once_cell::sync::Lazy;

pub static SEARCHBOX_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);


pub fn searchbox(content: &String) -> TextInput<'static, Message> {
    text_input("Search...", content)
            .on_input(Message::Search)
            .on_submit(Message::LaunchSelected)
            .width(Length::Fill)
            .id(SEARCHBOX_ID.clone())
}