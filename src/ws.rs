use std::thread;

use message_io::node::{self};
use message_io::network::{NetEvent, Transport};

use crate::com;

pub fn run() -> com::Sender {
    println!("Starting WebSocket server");

    let (handler, listener) = node::split::<()>();

    let (tx, rx) = com::create();

    // Listen for TCP, UDP and WebSocket messages at the same time.
    handler.network().listen(Transport::Ws, "0.0.0.0:3044").unwrap();

    thread::spawn(|| {
        // Read incoming network events.
        listener.for_each(move |event| match event.network() {
            NetEvent::Connected(_, _) => unreachable!(), // Used for explicit connections.
            NetEvent::Accepted(endpoint, _listener) => {
                println!("Client ({}) connected", endpoint.addr());

                for received in &rx {
                    let message = com::serialize(&received);
                    handler.network().send(endpoint.clone(), message.as_bytes());
                }
            },
            NetEvent::Message(_endpoint, data) => {
                println!("Received: {}", String::from_utf8_lossy(data));
            },
            NetEvent::Disconnected(endpoint) => {
                println!("Client ({}) disconnected", endpoint.addr());
            }
        });
    });

    tx
}