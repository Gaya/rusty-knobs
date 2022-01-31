use std::time::SystemTime;

mod lfo;

fn bmp_to_hz(bmp: u32) -> f64 {
    f64::from(bmp) / 60.0
}

fn main() {
    println!("LFO started");
    let now = SystemTime::now();

    loop {
        let t = now.elapsed().unwrap().as_secs_f64();

        println!("{}", lfo::calc_lfo(lfo::LFO::Sine, t, bmp_to_hz(120)));
    }
}