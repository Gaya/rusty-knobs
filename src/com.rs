use serde::{Serialize, Deserialize};

use std::sync::mpsc;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelMessage {
    pub sender: String,
    pub value: f64,
}

pub type Receiver = mpsc::Receiver<ChannelMessage>;
pub type Sender = mpsc::SyncSender<ChannelMessage>;

pub fn serialize(message: &ChannelMessage) -> String {
    serde_json::to_string(&message).unwrap()
}

pub fn create() -> (Sender, Receiver) {
    let (tx, rx) = mpsc::sync_channel::<ChannelMessage>(0);

    (tx, rx)
}