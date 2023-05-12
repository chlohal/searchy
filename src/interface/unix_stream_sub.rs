use std::{
    collections::VecDeque,
    hash::Hasher,
    io::{Read, self},
    os::unix::net::{UnixListener, UnixStream},
    pin::Pin,
    sync::{Arc, Mutex},
    task::Poll,
    thread,
};

use iced::{
    futures::{
        stream::{BoxStream, LocalBoxStream},
        Stream,
    },
    subscription::{self, Recipe},
    Event, Subscription,
};

use crate::ipc_communication::{message::IpcMessage, server_side::listen_socket};

use super::window::Message;

const UNIX_STREAM_IDENTIFIER: &str = "UNIX_STREAM";

pub enum State {
    Listening(UnixListener),
    Streaming(UnixListener, UnixStream),
    Done,
}

pub fn unix_stream_subscription() -> Subscription<Message> {
    if let Ok(listener) = listen_socket() {
        subscription::unfold(
            UNIX_STREAM_IDENTIFIER,
            State::Listening(listener),
            move |state| handle_stream(state),
        )
    } else {
        subscription::unfold(UNIX_STREAM_IDENTIFIER, State::Done, move |_| async {
            (Message::Ipc(IpcMessage::CloseProgram), State::Done)
        })
    }
}

async fn handle_stream(state: State) -> (Message, State) {
    match state {
        State::Listening(listener) => {
            listener
                .set_nonblocking(true)
                .expect("Couldn't set socket non-blocking");

            match listener.accept() {
                Ok((stream, _)) => (
                    Message::Ipc(IpcMessage::Refresh),
                    State::Streaming(listener, stream),
                ),
                Err(e) => {
                    match e.kind() {
                        io::ErrorKind::WouldBlock => {}
                        _ => eprintln!("{}", e.to_string()),
                    };

                    return (
                        Message::Ipc(IpcMessage::Refresh),
                        State::Listening(listener),
                    );
                }
            }
        }
        State::Streaming(listener, stream) => {
            if let Ok(m) = stream_to_ipc_message(stream) {
                return (Message::Ipc(m), State::Listening(listener));
            } else {
                return (
                    Message::Ipc(IpcMessage::Refresh),
                    State::Listening(listener),
                );
            }
        }
        State::Done => todo!(),
    }
}

fn stream_to_ipc_message(mut unix_stream: UnixStream) -> Result<IpcMessage, String> {
    let mut buf = Vec::<u8>::new();
    match unix_stream.read_to_end(&mut buf) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err.to_string());
            return Err(err.to_string());
        }
    };

    let message = match String::from_utf8(buf) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err.to_string());
            return Err(err.to_string());
        }
    };

    let ipc_message: IpcMessage = match message.try_into() {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err.to_string());
            return Err(err);
        }
    };

    Ok(ipc_message)
}
