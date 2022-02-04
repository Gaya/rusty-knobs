use std::sync::mpsc;

pub struct ChannelMessage {
    pub sender: String,
    pub value: f64,
}

pub type Receiver = mpsc::Receiver<ChannelMessage>;
pub type Sender = mpsc::Sender<ChannelMessage>;

pub fn create() -> (Sender, Receiver) {
    let (tx, rx) = mpsc::channel::<ChannelMessage>();

    (tx, rx)
}