pub enum LFO {
    Sine,
    Triangle,
    Sawtooth,
    Square,
    Random,
    RandomSmooth,
}

fn lfo_sine(t: f64, hz: f64) -> f64 {
    ((t * 2.0 * hz * std::f64::consts::PI).sin() + 1.0) / 2.0
}

pub fn calc_lfo(lfo: LFO, t: f64, hz: f64) -> f64 {
    match lfo {
        LFO::Sine => lfo_sine(t, hz),
        _ => 0.0,
    }
}