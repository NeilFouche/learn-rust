# Learn Rust
This repo holds a codebase for basic Digital Signal Processing (DSP) whose main
purpose is to give interesting context to learn and apply Rust techniques.

## Modules
### main:
  #### Build a circuit
  1. Compute equivalent resistance of two parallel resistors
  2. Calculate the resulting current
  #### Generates a signal
  1. Define a `square` waveform
  2. Create a periodic signal with the waveform
  3. Extract a piece of the signal
  #### Apply DSP
  1. Define an `integrator`
  2. Apply integrator to the signal
  3. Extract a piece of the signal with integrator applied
  #### Also test continuous signals
  1. Define a `random` response
  2. Create a continuous signal with a random response
  3. Extract a piece of the signal for testing

### circuits
Contains circuit elements and related functions

1. `enum` **Component**: For circuit elements
    - resistors
    - capacitors
    - inductors

    `method` **reactance**: Returns the reactance for resistor, capacitor
      and inductors respectively

2. `function` **parallel_resistance**: Returns the equivalent parallel resistance
  of two resistors
3. `function` **calculate_wavelength**: Returns the wavelength of a signal in
  a vacuum for a given frequency
4. `function` **get_max_value**:  Returns the maximum value of an input array
5. `function` **get_min_value**:  Returns the minimum value of an input array
6. `function` **get_minmax**:  Returns the minimum and maximum values of an input array
  as a tuple
7. `function` **normalize_signal**: Returns a new array from an input array with values
    mapped between 0 and 1.

### signals
Contains signals and related models.
1. `enum` **Waveform**: For different types of waveforms
    - Sine
    - Square
    - Triangle
    - Dc

    `method` **sample**: Returns the value of the waveform at time `t`

2. `enum` **Response**: The shape of the continuous signal
    - Ramp: Standard ramp signal with specified slope
    - Random: Signal with random values in a given range
    - UnitStep: Signal that is 1 for t > 0 and 0 otherwise

    * `method` **sample**: Returns the value of the waveform at time `t`

3. `struct` **DataSignal**\
    A signal whose values are set from an arbitrary input array

    * `method` **new**: Creates a new DataSignal
    * `method` **samples**: Returns the values of the signal as an array
    * `method` **len**: Number of samples in the signal
    * `method` **rms**: Calculates the root-mean-square value of the signal
    * `method` **normalize**: Returns the samples mapped between 0 and 1

4. `struct` **PeriodicSignal**\
    A signal whose values repeats periodically. The pattern is defined by its `waveform`.

    * `method` **new**: Creates a new PeriodicSignal
    * `method` **sample**: Returns the value of the signal at time `t`
    * `method` **interval**: Returns multiple values of the signal at time between
      start and end with a given step size.
    * `method` **add_filter**: Adds a filter to the signal. When the signal contains
      a filter, `sample` and `interval` will return filtered values.
    * `method` **frequency**: Returns the frequency of the waveform.
    * `method` **amplitude**: Returns the amplitude of the waveform.
5. `struct` **ContinuousSignal**\
    A continuous signal whose shape is defined by its `response`.

    Methods: Same as `PeriodicSignal` excluding `frequency` and `amplitude`.

### dsp:
Contains Digital Signal Processing elements.

1. `enum` **Filter**: For different types of filter
    - Differentiator { `tau:` `f64` }
    - HighPassFilter { `alpha:` `f64` }
    - Integrator { `tau:` `f64` }
    - LowPassFilter { `alpha:` `f64` }
    - Rectifier

    `method` **apply**: Applies the filter function to the sample of a signal

