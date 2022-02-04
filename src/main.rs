use std::thread;
use std::time::SystemTime;

mod lfo;
mod ws;
mod com;

fn bmp_to_hz(bmp: u32) -> f64 {
    f64::from(bmp) / 60.0
}

fn main() {
    // setup
    let (tx, rx) = com::create();
    let now = SystemTime::now();
    ws::run(rx);

    // settings
    let lfo_type: lfo::LFO = lfo::LFO::Sine;
    let hz = bmp_to_hz(120);

    println!("LFO started");

    loop {
        let t = now.elapsed().unwrap().as_secs_f64();

        let value = lfo::calc(lfo_type, t, hz);

        let sender = String::from("lfo");
        let message = com::ChannelMessage { sender, value };
        tx.send(message).unwrap();
    }
}