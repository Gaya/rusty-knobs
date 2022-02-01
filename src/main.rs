use std::time::SystemTime;

mod lfo;

fn bmp_to_hz(bmp: u32) -> f64 {
    f64::from(bmp) / 60.0
}

fn main() {
    // setup
    let now = SystemTime::now();

    // settings
    let lfo_type: lfo::LFO = lfo::LFO::Sine;
    let hz = bmp_to_hz(120);

    println!("LFO started");

    loop {
        let t = now.elapsed().unwrap().as_secs_f64();

        println!("{}", lfo::calc(lfo_type, t, hz));
    }
}