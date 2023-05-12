#[derive(Debug, Clone)]
pub enum IpcMessage {
    OpenWindow,
    CloseProgram,
    Refresh
}

const OPEN_WINDOW: &'static str = "open-window";
const CLOSE_PROGRAM: &'static str = "close-program";
const REFRESH: &'static str = "refresh";

impl Into<&str> for IpcMessage {
    fn into(self) -> &'static str {
        match self {
            IpcMessage::OpenWindow => OPEN_WINDOW,
            IpcMessage::CloseProgram => CLOSE_PROGRAM,
            IpcMessage::Refresh => REFRESH,
        }
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