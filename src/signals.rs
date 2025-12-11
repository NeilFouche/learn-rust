//! Contains signals and related models.

use crate::circuits::normalize_signal;
use crate::dsp::Filter;
use rand::Rng;

pub enum Waveform {
    Sine { frequency: f64, amplitude: f64 },
    Square { frequency: f64, amplitude: f64 },
    Triangle { frequency: f64, amplitude: f64 },
    Dc(f64),
}

impl Waveform {
    pub fn sample(&self, t: f64) -> f64 {
        match self {
            Waveform::Sine {
                frequency,
                amplitude,
            } => amplitude * (2.0 * std::f64::consts::PI * frequency * t).sin(),
            Waveform::Square {
                frequency,
                amplitude,
            } => {
                let phase = (t * frequency) % 1.0;
                if phase < 0.5 { *amplitude } else { -*amplitude }
            }
            Waveform::Triangle {
                frequency,
                amplitude,
            } => {
                let phase = (t * frequency) % 1.0; // Measures how far t is along cycle
                if phase < 0.5 {
                    // y = mx-c = 4A(phase) - A
                    4.0 * amplitude * phase - amplitude // Rising edge (0, -A) to (0.5, A)
                } else {
                    // z = 3A - y
                    3.0 * amplitude - 4.0 * amplitude * phase // Falling edge (0.5, A) to (1, -A)
                }
            }
            Waveform::Dc(amplitude) => *amplitude,
        }
    }
}

pub enum Response {
    // Bell { amplitude: f64, sigma: f64 },
    // Exponential { tau: f64 },
    Ramp { slope: f64 },
    Random { min: f64, max: f64 },
    // Sinc { amplitude: f64 },
    UnitStep,
    // UnitImpulse {},
}

impl Response {
    pub fn sample(&self, t: f64) -> f64 {
        match self {
            Response::Ramp { slope } => slope * t,
            Response::Random { min, max } => {
                let mut rng = rand::rng();
                rng.random_range(*min..*max)
            }
            Response::UnitStep => {
                if t >= 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }
}

pub struct DataSignal {
    samples: Vec<f64>,
}

impl DataSignal {
    /// Creates a new `Signal` from sample data and a sample rate.
    pub fn new(samples: Vec<f64>) -> Self {
        Self { samples }
    }

    /// Returns a read-only view of the samples.
    pub fn samples(&self) -> &[f64] {
        &self.samples
    }

    /// Returns the number of samples.
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Computes the RMS (root-mean-square) value of the signal.
    pub fn rms(&self) -> f64 {
        let sum_sq: f64 = self.samples.iter().map(|x| x * x).sum();
        (sum_sq / self.len() as f64).sqrt()
    }

    /// Normalizes the signal so that its maximum absolute value becomes 1.
    pub fn normalize(&mut self) {
        self.samples = normalize_signal(&self.samples);
    }
}

pub struct PeriodicSignal {
    waveform: Waveform,
    filter: Option<Filter>,
    previous_input: f64,
    previous_output: f64,
}

impl PeriodicSignal {
    pub fn new(waveform: Waveform) -> Self {
        Self {
            waveform,
            filter: None,
            previous_output: 0.0,
            previous_input: 0.0,
        }
    }

    pub fn sample(&mut self, t: f64) -> f64 {
        match &self.filter {
            Some(filter) => {
                let input = self.waveform.sample(t);
                let filtered_sample =
                    filter.apply(input, self.previous_output, self.previous_input);
                self.previous_output = filtered_sample;
                self.previous_input = input;
                filtered_sample
            }
            None => self.waveform.sample(t),
        }
    }

    pub fn interval(&mut self, start: f64, end: f64, step: f64) -> Vec<f64> {
        let mut samples: Vec<f64> = Vec::new();
        let mut t = start;

        while t <= end {
            samples.push(self.sample(t));
            t += step;
        }

        samples
    }

    pub fn add_filter(&mut self, filter: Filter) {
        self.filter = Some(filter);
    }

    pub fn frequency(&self) -> f64 {
        match &self.waveform {
            Waveform::Sine { frequency, .. } => *frequency,
            Waveform::Square { frequency, .. } => *frequency,
            Waveform::Triangle { frequency, .. } => *frequency,
            Waveform::Dc(..) => 0.0,
        }
    }

    pub fn amplitude(&self) -> f64 {
        match &self.waveform {
            Waveform::Sine { amplitude, .. } => *amplitude,
            Waveform::Square { amplitude, .. } => *amplitude,
            Waveform::Triangle { amplitude, .. } => *amplitude,
            Waveform::Dc(amplitude) => *amplitude,
        }
    }
}

pub struct ContinuousSignal {
    response: Response,
    filter: Option<Filter>,
    previous_output: f64,
    previous_input: f64,
}

impl ContinuousSignal {
    pub fn new(response: Response) -> Self {
        Self {
            response,
            filter: None,
            previous_output: 0.0,
            previous_input: 0.0,
        }
    }

    pub fn sample(&mut self, t: f64) -> f64 {
        match &self.filter {
            Some(filter) => {
                let input = self.response.sample(t);
                let filtered_sample =
                    filter.apply(input, self.previous_output, self.previous_input);
                self.previous_output = filtered_sample;
                self.previous_input = input;
                filtered_sample
            }
            None => self.response.sample(t),
        }
    }

    pub fn interval(&mut self, start: f64, end: f64, step: f64) -> Vec<f64> {
        let mut samples: Vec<f64> = Vec::new();
        let mut t = start;

        while t <= end {
            samples.push(self.sample(t));
            t += step;
        }

        samples
    }

    pub fn add_filter(&mut self, filter: Filter) {
        self.filter = Some(filter);
    }
}
