use std::{
    io::Read,
    os::unix::net::{UnixListener, UnixStream}, thread,
};

use iced::{
    subscription, Subscription,
};
use iced_native::futures::{
    channel::mpsc::{channel, Sender},
    executor::block_on, SinkExt,
};

use crate::ipc_communication::{message::IpcMessage, server_side::listen_socket};

use super::window::Message;

pub fn unix_stream_subscription() -> Subscription<Message> {
    subscription::run(stream_builder)
}

fn stream_builder() -> iced_native::futures::channel::mpsc::Receiver<Message> {
    if let Ok(listener) = listen_socket() {
        let (send, recieve) = channel::<Message>(10);
        accept_into_queue(listener, send);

        recieve
    } else {
        channel::<Message>(1).1
    }
}

fn accept_into_queue(listener: UnixListener, mut message_queue: Sender<Message>) {
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream_to_ipc_message(stream.unwrap()) {
                Ok(ipc) => {
                    block_on(message_queue.send(Message::Ipc(ipc))).unwrap();
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    });
}

fn stream_to_ipc_message(mut unix_stream: UnixStream) -> Result<IpcMessage, String> {
    let mut buf = Vec::<u8>::new();
    match unix_stream.read_to_end(&mut buf) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err);
            return Err(err.to_string());
        }
    };

    let message = match String::from_utf8(buf) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err);
            return Err(err.to_string());
        }
    };

    let ipc_message: IpcMessage = match message.try_into() {
        Ok(it) => it,
        Err(err) => {
            eprintln!("{}", err);
            return Err(err);
        }
    };

    Ok(ipc_message)
}
