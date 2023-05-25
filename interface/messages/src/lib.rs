use std::sync::Arc;

use actions::actions::Action;
use ipc_communication::message::IpcMessage;

#[derive(Debug, Clone)]
pub enum Message {
    Search(String),
    Ipc(IpcMessage),
    ResultMessage(SearchResultMessage),
    HideWindow,
    GenericKey,
}

#[derive(Debug, Clone)]
pub enum SearchResultMessage {
    SelectNext,
    SelectPrevious,
    Scroll(f32),
    Search(String),
    ClickOption(Arc<Action>),
    LaunchSelected
}