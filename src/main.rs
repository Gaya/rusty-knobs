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
    let now = SystemTime::now();
    let tx = ws::run();

    // settings
    let lfo_type: lfo::LFO = lfo::LFO::Sine;
    let hz = bmp_to_hz(120) / 4.0;
    let per_second = 120;

    println!("LFO started");

    let mut prev_t = now.elapsed().unwrap().as_secs_f64();
    loop {
        let t = now.elapsed().unwrap().as_secs_f64();

        if (prev_t - t).abs() > 1.0 / per_second as f64 {
            let value = lfo::calc(lfo_type, t, hz);

            let sender = String::from("lfo");
            let message = com::ChannelMessage { sender, value };
            tx.send(message).unwrap();

            prev_t = t;
        }
    }
}