use std::collections::{HashMap};
use std::sync::mpsc;
use std::thread;

use message_io::node::{self};
use message_io::network::{NetEvent, Transport, Endpoint};

use crate::com;
use crate::com::Sender;

pub fn run() -> com::Sender {
    println!("Starting WebSocket server");

    let mut clients: HashMap<Endpoint, Endpoint> = HashMap::new();
    let (handler, listener) = node::split::<()>();

    let (tx, rx) = com::create();

    // Listen for TCP, UDP and WebSocket messages at the same time.
    handler.network().listen(Transport::Ws, "0.0.0.0:3044").unwrap();

    thread::spawn(|| {
        // Read incoming network events.
        listener.for_each(move |event| match event.network() {
            NetEvent::Connected(_, _) => unreachable!(), // Used for explicit connections.
            NetEvent::Accepted(endpoint, _listener) => {
                clients.insert(endpoint, endpoint);
                println!("Client ({}) connected (total clients: {})", endpoint.addr(), clients.len());

                for received in &rx {
                    let message = com::serialize(&received);
                    handler.network().send(endpoint, message.as_bytes());
                }
            },
            NetEvent::Message(_endpoint, data) => {
                println!("Received: {}", String::from_utf8_lossy(data));
            },
            NetEvent::Disconnected(endpoint) => {
                // Only connection oriented protocols will generate this event
                clients.remove(&endpoint).unwrap();
                println!(
                    "Client ({}) disconnected (total clients: {})",
                    endpoint.addr(),
                    clients.len()
                );
            }
        });
    });

    tx
}