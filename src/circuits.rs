//! Contains circuit elements and related functions

const SPEED_OF_LIGHT: f64 = 299_792_458.0; // m/s

pub enum Component {
    Resistor(f64),  // Resistance in ohms
    Capacitor(f64), // Capacitance in farads
    Inductor(f64),  // Inductance in henries
}

impl Component {
    pub fn reactance(&self, frequency: f64) -> f64 {
        match self {
            Component::Resistor(r) => *r,
            Component::Capacitor(c) => -1.0 / (2.0 * std::f64::consts::PI * frequency * c),
            Component::Inductor(l) => 2.0 * std::f64::consts::PI * frequency * l,
        }
    }
}

/// Calculates the equivalent parallel resistance us r_eq = (r1*r2)/(r1+r2).
pub fn parallel_resistance(r1: f64, r2: f64) -> f64 {
    let numerator = r1 * r2;
    let denominator = r1 + r2;

    numerator / denominator
}

/// Calculates wavelength (λ) from frequency (f) using λ = c / f.\
/// where c = speed of light (299,792,458 m/s)
pub fn calculate_wavelength(frequency: f64) -> f64 {
    SPEED_OF_LIGHT / frequency // using constants
}

fn get_max_value(values: &[f64]) -> f64 {
    let max_value = values.iter().cloned().fold(f64::MIN, f64::max);
    max_value
}

fn get_min_value(values: &[f64]) -> f64 {
    let min_value = values.iter().cloned().fold(f64::MAX, f64::min);
    min_value
}

pub fn get_minmax(values: &[f64]) -> (f64, f64) {
    let max_value = get_max_value(values);
    let min_value = get_min_value(values);
    (min_value, max_value) // return as a tuple
}

/// Normalizes the input signal to the range [0, 1].
pub fn normalize_signal(signal: &[f64]) -> Vec<f64> {
    let mut output: Vec<f64> = Vec::with_capacity(signal.len());
    let (min_value, max_value) = get_minmax(signal); // destructuring

    for n in 0..signal.len() {
        let value = signal[n];
        let value = (value - min_value) / (max_value - min_value); // shadowing value
        output.push(value);
    }

    output
}
