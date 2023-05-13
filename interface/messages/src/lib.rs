use std::sync::Arc;

use actions::actions::Action;
use ipc_communication::message::IpcMessage;

#[derive(Debug, Clone)]
pub enum Message {
    LaunchSelected,
    ClickOption(Arc<Action>),
    SelectNext,
    SelectPrevious,
    Search(String),
    Scroll(f32),
    Ipc(IpcMessage),
    HideWindow,
}