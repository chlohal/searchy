use iced::keyboard::{Event, KeyCode};
use ipc_communication::message::IpcMessage::Javascript;
use match_friendly_modifier::{match_friendly_modifier, NONE, CTRL, CTRL_SHIFT};
use messages::{Message, SearchResultMessage};

mod match_friendly_modifier;

pub fn handle_key_event(e: Event) -> Option<Message> {
    let Event::KeyPressed {key_code, modifiers} = e else { return None; };

    print!("{:?}", key_code);

    match (match_friendly_modifier(modifiers), key_code) {
        (_, KeyCode::Escape) => Some(Message::HideWindow),
        (NONE, KeyCode::Down) => Some(Message::ResultMessage(SearchResultMessage::SelectNext)),
        (NONE, KeyCode::Up) => Some(Message::ResultMessage(SearchResultMessage::SelectPrevious)),
        (CTRL, KeyCode::Grave) => Some(Message::Ipc(Javascript)),
        (CTRL, KeyCode::Enter) => Some(Message::ExecuteTypeShell),
        (CTRL_SHIFT, KeyCode::Enter) => Some(Message::ExecuteTypeTerminal),
        (NONE, KeyCode::Enter) => Some(Message::ResultMessage(SearchResultMessage::LaunchSelected)),
        _ => None,
    }
}
