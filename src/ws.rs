use std::collections::{HashMap};
use std::thread;

use message_io::node::{self};
use message_io::network::{NetEvent, Transport, Endpoint};

mod com;

struct ClientInfo {
    count: usize,
}

pub fn run(rx: com::Receiver) {
    println!("Starting WebSocket server");

    let mut clients: HashMap<Endpoint, ClientInfo> = HashMap::new();

    thread::spawn(|| {
        let (handler, listener) = node::split::<()>();

        // Listen for TCP, UDP and WebSocket messages at the same time.
        handler.network().listen(Transport::Ws, "0.0.0.0:3044").unwrap();

        // Read incoming network events.
        listener.for_each(move |event| match event.network() {
            NetEvent::Connected(_, _) => unreachable!(), // Used for explicit connections.
            NetEvent::Accepted(endpoint, _listener) => {
                clients.insert(endpoint, ClientInfo { count: 0 });
                println!("Client ({}) connected (total clients: {})", endpoint.addr(), clients.len());
            },
            NetEvent::Message(endpoint, data) => {
                println!("Received: {}", String::from_utf8_lossy(data));
                handler.network().send(endpoint, data);
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

    thread::spawn(|| {
        for received in rx {
            println!("Receive: {}", received);
        }
    });
}