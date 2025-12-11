//! Contains Digital Signal Processing elements.

pub enum Filter {
    Differentiator { tau: f64 },
    Integrator { tau: f64 },
    HighPassFilter { alpha: f64 },
    LowPassFilter { alpha: f64 },
    Rectifier,
}

impl Filter {
    pub fn apply(&self, input: f64, previous_output: f64, previous_input: f64) -> f64 {
        match self {
            Filter::Differentiator { tau } => (input - previous_input) / tau,
            Filter::Integrator { tau } => tau * input + previous_output,
            Filter::HighPassFilter { alpha } => {
                input - (alpha * (previous_input - previous_output) + input * (1.0 - alpha))
            }
            Filter::LowPassFilter { alpha } => (1.0 - alpha) * input + alpha * previous_output,
            Filter::Rectifier => input.abs(),
        }
    }
}
