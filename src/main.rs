//! The main purpose of this package is to serve as a learning playground for Rust programming.
//! As such it contains basically a module for each chapter of the Rust book.
//! The functions and structs demonstrate various Rust programming concepts while still being
//! real and useful. This program aims to touch each of those modules.

mod circuits;
mod dsp;
mod signals;

pub fn main() {
    // Build circuit
    let r_eq = circuits::parallel_resistance(10.0, 10.0);
    let voltage = 5.0;
    let current = voltage / r_eq;
    println!("Equivalent parallel resistance: {r_eq} Ω");
    println!("Current through the circuit: {current} A");

    // Generate a signal
    let frequency = 2.0; // Hz
    let wavelength = circuits::calculate_wavelength(frequency);
    println!("Wavelength at {frequency} Hz: {wavelength} m");

    let waveform = signals::Waveform::Square {
        frequency,
        amplitude: current,
    };
    let mut signal = signals::PeriodicSignal::new(waveform);
    let pure_signal = signal.interval(0.0, 1.0, 1.0 / 360.0);
    println!("Display pure values: {:?}", &pure_signal);

    // Apply DSP: low-pass filter
    let integrator = dsp::Filter::Integrator {
        tau: 2.0 * frequency / pure_signal.len() as f64,
    };
    signal.add_filter(integrator);
    let filtered_signal = signal.interval(0.0, 1.0, 1.0 / 360.0);
    println!("Filtered signal: {:?}", filtered_signal);

    // Generate continuous signal
    let response = signals::Response::Random {
        min: -1.0,
        max: 1.0,
    };
    let mut continuous_signal = signals::ContinuousSignal::new(response);
    let test_frame = continuous_signal.interval(0.0, 10.0, 1.0);
    println!("Continuous signal test set: {:?}", test_frame);
}
