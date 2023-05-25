use iced::keyboard::{Event, KeyCode};
use match_friendly_modifier::{match_friendly_modifier, ALT, NONE};
use messages::{Message, SearchResultMessage};

mod match_friendly_modifier;

pub fn handle_key_event(e: Event) -> Option<Message> {
    let Event::KeyPressed {key_code, modifiers} = e else { return None; };

    match (match_friendly_modifier(modifiers), key_code) {
        (ALT, KeyCode::F4) => {
            if modifiers.alt() {
                Some(Message::HideWindow)
            } else {
                None
            }
        }
        (_, KeyCode::Escape) => Some(Message::HideWindow),
        (NONE, KeyCode::Down) => Some(Message::ResultMessage(SearchResultMessage::SelectNext)),
        (NONE, KeyCode::Up) => Some(Message::ResultMessage(SearchResultMessage::SelectPrevious)),
        _ => Some(Message::GenericKey),
    }
}
