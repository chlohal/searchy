pub enum IpcMessage {
    OpenWindow
}

const OPEN_WINDOW: &'static str = "open-window";

impl Into<&str> for IpcMessage {
    fn into(self) -> &'static str {
        match self {
            IpcMessage::OpenWindow => OPEN_WINDOW,
        }
    }
}


impl TryFrom<String> for IpcMessage {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            OPEN_WINDOW => Ok(IpcMessage::OpenWindow),
            v => Err(format!("Could not parse IPC message from {}", v))
        }
    }
}