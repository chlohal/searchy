use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum IpcMessage {
    OpenWindow,
    CloseProgram,
    Refresh,
    AppSearch,
    Javascript
}

const OPEN_WINDOW: &str = "open-window";
const CLOSE_PROGRAM: &str = "close-program";
const REFRESH: &str = "refresh";
const APP_SEARCH: &str = "app-search";
const JAVASCRIPT: &str = "javascript";

impl From<IpcMessage> for &str {
    fn from(val: IpcMessage) -> Self {
        match val {
            IpcMessage::OpenWindow => OPEN_WINDOW,
            IpcMessage::CloseProgram => CLOSE_PROGRAM,
            IpcMessage::Refresh => REFRESH,
            IpcMessage::AppSearch => APP_SEARCH,
            IpcMessage::Javascript => JAVASCRIPT,
        }
    }
}

impl Display for IpcMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Into::<&str>::into(*self))
    }
}

impl TryFrom<String> for IpcMessage {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            OPEN_WINDOW => Ok(IpcMessage::OpenWindow),
            CLOSE_PROGRAM => Ok(IpcMessage::CloseProgram),
            REFRESH => Ok(IpcMessage::Refresh),
            v => Err(format!("Could not parse IPC message from {}", v))
        }
    }
}