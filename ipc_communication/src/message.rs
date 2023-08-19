use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum IpcMessage {
    OpenWindow,
    CloseProgram,
    Refresh,
    AppSearch,
    Javascript
}

macro_rules! enum_parsable {
    { $typ:path => { $( $a:path => $b:literal ),* } }   => {
        impl From<$typ> for &str {
            fn from(val: $typ) -> Self {
                match val {
                    $(
                       $a => $b,
                    )*
                }
            }
        }

        impl Display for $typ {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(Into::<&str>::into(*self))
            }
        }

        impl TryFrom<String> for $typ {
            type Error = String;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                match value.as_str() {
                    $(
                        $b => Ok($a),
                     )*
                     v => Err(format!("Could not parse IPC message from {}", v)),
                }
            }
        }

        impl $typ {
            pub fn representations() -> Vec<&'static str> {
                vec![
                    $( $b, )*
                ]
            }
        }

    };
}

enum_parsable! {
    IpcMessage => {
        IpcMessage::OpenWindow => "open-window",
        IpcMessage::CloseProgram => "close-program",
        IpcMessage::Refresh => "refresh",
        IpcMessage::AppSearch => "app-search",
        IpcMessage::Javascript => "javascript"
    }
}