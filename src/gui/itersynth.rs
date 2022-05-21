// +--------------------------------------------------------------------------+
// | Copyright 2016 Matthew D. Steele <mdsteele@alum.mit.edu>                 |
// |                                                                          |
// | This file is part of System Syzygy.                                      |
// |                                                                          |
// | System Syzygy is free software: you can redistribute it and/or modify it |
// | under the terms of the GNU General Public License as published by the    |
// | Free Software Foundation, either version 3 of the License, or (at your   |
// | option) any later version.                                               |
// |                                                                          |
// | System Syzygy is distributed in the hope that it will be useful, but     |
// | WITHOUT ANY WARRANTY; without even the implied warranty of               |
// | MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU        |
// | General Public License for details.                                      |
// |                                                                          |
// | You should have received a copy of the GNU General Public License along  |
// | with System Syzygy.  If not, see <http://www.gnu.org/licenses/>.         |
// +--------------------------------------------------------------------------+

use std::f32::consts::PI;
use std::ops::{Add, Mul};

// ========================================================================= //

/// One sample value from a waveform.
///
/// When a `Sample` is used as a [`WaveGen`](trait.WaveGen.html), it generates
/// a constant value forever.
pub type Sample = f32;

/// A waveform generator.
pub trait WaveGen: Send {
    /// Gets the next sample value, or returns `None` if the waveform has
    /// finished.  The `step` gives the number of seconds to advance.
    fn next(&mut self, step: f32) -> Option<Sample>;

    /// Resets the waveform back to the beginning.
    fn reset(&mut self);

    /// Clones the `WaveGen` and wraps it as a `Wave`.
    fn as_wave(&self) -> Wave;
}

impl WaveGen for Sample {
    fn next(&mut self, _: f32) -> Option<Sample> {
        Some(*self)
    }

    fn reset(&mut self) {}

    fn as_wave(&self) -> Wave {
        Wave::new(Box::new(*self))
    }
}

// ========================================================================= //

/// A sequence of sample values, forming a waveform.
///
/// Normally the samples will represent sound amplitude values, but they can
/// also represent frequencies or other values, depending on context.
pub struct Wave {
    generator: Box<dyn WaveGen>,
}

impl Wave {
    /// Creates a waveform using the given generator.
    pub fn new(generator: Box<dyn WaveGen>) -> Wave {
        Wave { generator }
    }

    /// Creates a noise wave, with an amplitude of 1, whose frequency over time
    /// is controlled by the input waveform (which may be a constant).  The
    /// input frequency values are measured in hertz (cycles per second).
    pub fn noise<F: Into<Wave>>(freq: F) -> Wave {
        Wave::new(Box::new(NoiseWave::new(freq.into())))
    }

    /// Creates a pulse wave whose frequency over time is controlled by the
    /// `freq` waveform, and whose duty cycle over time is controlled by the
    /// `duty` waveform (either or both of which may be constants).  The input
    /// frequency values are measured in hertz (cycles per second); the input
    /// duty values should be between 0 and 1 (with 0.5 being a square wave).
    pub fn pulse<F: Into<Wave>, D: Into<Wave>>(freq: F, duty: D) -> Wave {
        Wave::new(Box::new(PulseWave::new(freq.into(), duty.into())))
    }

    /// Creates a sine wave, with an amplitude of 1, whose frequency over time
    /// is controlled by the input waveform (which may be a constant).  The
    /// input frequency values are measured in hertz (cycles per second).
    pub fn sine<F: Into<Wave>>(freq: F) -> Wave {
        Wave::new(Box::new(SineWave::new(freq.into())))
    }

    /// Creates a wave with the shape of a parabola; it's initial value is
    /// `pos`, it's initial velocity is `vel` (units/second), and it
    /// accelerates with `acc` (units/second/second).  Generally not useful as
    /// a sound wave, but can be used to control e.g. the frequency of another
    /// wave.
    pub fn slide(pos: f32, vel: f32, acc: f32) -> Wave {
        Wave::new(Box::new(SlideWave::new(pos, vel, acc)))
    }

    /// Creates a triangle wave whose frequency over time is controlled by the
    /// `freq` waveform, and whose duty cycle over time is controlled by the
    /// `duty` waveform (either or both of which may be constants).  The input
    /// frequency values are measured in hertz (cycles per second); the input
    /// duty values should be between 0 and 1 (with 0.5 being a triangle wave
    /// and 0 or 1 being a sawtooth wave).
    pub fn triangle<F: Into<Wave>, D: Into<Wave>>(freq: F, duty: D) -> Wave {
        Wave::new(Box::new(TriangleWave::new(freq.into(), duty.into())))
    }

    /// Returns a new waveform that delays this one for a duration.
    pub fn delayed(self, seconds: f32) -> Wave {
        Wave::new(Box::new(Delayed::new(self, seconds)))
    }

    /// Returns a new waveform that repeats this one a fixed number of times.
    pub fn repeated(self, num_times: i32) -> Wave {
        Wave::new(Box::new(Repeated::new(self, num_times)))
    }

    /// Returns a new waveform by constraining this one with an ADSHR (attack,
    /// decay, sustain, hold, release) envelope.
    pub fn adshr(
        self,
        attack_time: f32,
        decay_time: f32,
        sustain_level: f32,
        hold_time: f32,
        release_time: f32,
    ) -> Wave {
        Wave::new(Box::new(Adshr {
            attack_time,
            decay_time,
            sustain_level,
            hold_time,
            release_time,
            time: 0.0,
        })) * self
    }
}

impl<W: Into<Wave>> Add<W> for Wave {
    type Output = Wave;

    fn add(self, rhs: W) -> Wave {
        Wave::new(Box::new(Sum { wave1: self, wave2: rhs.into() }))
    }
}

impl Clone for Wave {
    fn clone(&self) -> Wave {
        self.generator.as_wave()
    }
}

impl From<Sample> for Wave {
    fn from(sample: Sample) -> Wave {
        Wave::new(Box::new(sample))
    }
}

impl<W: Into<Wave>> Mul<W> for Wave {
    type Output = Wave;

    fn mul(self, rhs: W) -> Wave {
        Wave::new(Box::new(Product { wave1: self, wave2: rhs.into() }))
    }
}

impl WaveGen for Wave {
    fn next(&mut self, step: f32) -> Option<Sample> {
        self.generator.next(step)
    }

    fn reset(&mut self) {
        self.generator.reset();
    }

    fn as_wave(&self) -> Wave {
        self.generator.as_wave()
    }
}

// ========================================================================= //

/// A waveform representing an ADSHR (attack, decay, sustain, hold, release)
/// envelope.
#[derive(Clone)]
struct Adshr {
    attack_time: f32,
    decay_time: f32,
    sustain_level: f32,
    hold_time: f32,
    release_time: f32,
    time: f32,
}

impl WaveGen for Adshr {
    fn next(&mut self, step: f32) -> Option<Sample> {
        let time = self.time;
        let value = if time < self.attack_time {
            time / self.attack_time
        } else {
            let time = time - self.attack_time;
            if time < self.decay_time {
                1.0 - (time / self.decay_time) * (1.0 - self.sustain_level)
            } else {
                let time = time - self.decay_time;
                if time < self.hold_time {
                    self.sustain_level
                } else {
                    let time = time - self.hold_time;
                    if time < self.release_time {
                        (1.0 - time / self.release_time) * self.sustain_level
                    } else {
                        return None;
                    }
                }
            }
        };
        self.time += step;
        Some(value)
    }

    fn reset(&mut self) {
        self.time = 0.0
    }

    fn as_wave(&self) -> Wave {
        Wave::new(Box::new(self.clone()))
    }
}

// ========================================================================= //

/// A waveform consisting of some other waveform delayed by a fixed duration.
#[derive(Clone)]
struct Delayed {
    wave: Wave,
    delay: f32,
    time: f32,
}

impl Delayed {
    fn new(wave: Wave, delay: f32) -> Delayed {
        Delayed { wave, delay, time: 0.0 }
    }
}

impl WaveGen for Delayed {
    fn next(&mut self, step: f32) -> Option<Sample> {
        if self.time >= self.delay {
            self.wave.next(step)
        } else {
            self.time += step;
            if self.time > self.delay {
                // Advance wave but ignore result.
                self.wave.next(self.time - self.delay);
            }
            None
        }
    }

    fn reset(&mut self) {
        self.wave.reset();
        self.time = 0.0;
    }

    fn as_wave(&self) -> Wave {
        Wave::new(Box::new(self.clone()))
    }
}

// ========================================================================= //

const NOISE_INIT_SEED: u64 = 123456789123456789;

/// A variable-frequency noise wave, with an amplitude of 1.
#[derive(Clone)]
struct NoiseWave {
    freq: Wave,
    seed: u64,
    phase: f32,
}

impl NoiseWave {
    fn new(freq: Wave) -> NoiseWave {
        NoiseWave { freq, seed: NOISE_INIT_SEED, phase: 0.0 }
    }
}

impl WaveGen for NoiseWave {
    fn next(&mut self, step: f32) -> Option<Sample> {
        let freq = match self.freq.next(step) {
            Some(freq) => freq,
            None => return None,
        };
        let phase = self.phase;
        let seed = self.seed;
        self.phase += 2.0 * freq * step;
        if self.phase >= 64.0 {
            self.phase %= 64.0;
            // This is a simple linear congruential generator, using parameters
            // suggested by http://nuclear.llnl.gov/CNP/rng/rngman/node4.html
            self.seed = self.seed.overflowing_mul(2862933555777941757).0;
            self.seed = self.seed.overflowing_add(3037000493u64).0;
        }
        Some(if ((seed >> (phase as i32)) & 1) != 0 { 1.0 } else { -1.0 })
    }

    fn reset(&mut self) {
        self.freq.reset();
        self.seed = NOISE_INIT_SEED;
        self.phase = 0.0;
    }

    fn as_wave(&self) -> Wave {
        Wave::new(Box::new(self.clone()))
    }
}

// ========================================================================= //

/// A waveform consisting of the product of two other waveforms.
#[derive(Clone)]
struct Product {
    wave1: Wave,
    wave2: Wave,
}

impl WaveGen for Product {
    fn next(&mut self, step: f32) -> Option<Sample> {
        match self.wave1.next(step) {
            Some(value1) => match self.wave2.next(step) {
                Some(value2) => Some(value1 * value2),
                None => None,
            },
            None => None,
        }
    }

    fn reset(&mut self) {
        self.wave1.reset();
        self.wave2.reset();
    }

    fn as_wave(&self) -> Wave {
        Wave::new(Box::new(self.clone()))
    }
}

// ========================================================================= //

/// A variable-frequency, variable-duty pulse wave, with an amplitude of 1.
#[derive(Clone)]
struct PulseWave {
    freq: Wave,
    duty: Wave,
    phase: f32,
}

impl PulseWave {
    fn new(freq: Wave, duty: Wave) -> PulseWave {
        PulseWave { freq, duty, phase: 0.0 }
    }
}

impl WaveGen for PulseWave {
    fn next(&mut self, step: f32) -> Option<Sample> {
        let freq = match self.freq.next(step) {
            Some(freq) => freq,
            None => return None,
        };
        let duty = match self.duty.next(step) {
            Some(duty) => duty,
            None => return None,
        };
        let phase = self.phase;
        self.phase = (self.phase + freq * step) % 1.0;
        Some(if phase < duty { 1.0 } else { -1.0 })
    }

    fn reset(&mut self) {
        self.freq.reset();
        self.duty.reset();
        self.phase = 0.0;
    }

    fn as_wave(&self) -> Wave {
        Wave::new(Box::new(self.clone()))
    }
}

// ========================================================================= //

/// A waveform consisting of some other waveform, repeated a fixed number of
/// times.
#[derive(Clone)]
struct Repeated {
    wave: Wave,
    num_times: i32,
    count: i32,
}

impl Repeated {
    fn new(wave: Wave, num_times: i32) -> Repeated {
        Repeated { wave, num_times, count: 0 }
    }
}

impl WaveGen for Repeated {
    fn next(&mut self, step: f32) -> Option<Sample> {
        while self.count < self.num_times {
            if let Some(sample) = self.wave.next(step) {
                return Some(sample);
            }
            self.wave.reset();
            self.count += 1;
        }
        None
    }

    fn reset(&mut self) {
        self.wave.reset();
        self.count = 0;
    }

    fn as_wave(&self) -> Wave {
        Wave::new(Box::new(self.clone()))
    }
}

// ========================================================================= //

/// A variable-frequency sine wave, with an amplitude of 1.
#[derive(Clone)]
struct SineWave {
    freq: Wave,
    phase: f32,
}

impl SineWave {
    fn new(freq: Wave) -> SineWave {
        SineWave { freq, phase: 0.0 }
    }
}

impl WaveGen for SineWave {
    fn next(&mut self, step: f32) -> Option<Sample> {
        let freq = match self.freq.next(step) {
            Some(freq) => freq,
            None => return None,
        };
        let phase = self.phase;
        self.phase = (self.phase + freq * step) % 1.0;
        Some((2.0 * PI * phase).sin())
    }

    fn reset(&mut self) {
        self.freq.reset();
        self.phase = 0.0;
    }

    fn as_wave(&self) -> Wave {
        Wave::new(Box::new(self.clone()))
    }
}

// ========================================================================= //

/// A parabolic wave.
#[derive(Clone)]
struct SlideWave {
    pos: f32,
    vel: f32,
    half_acc: f32,
    time: f32,
}

impl SlideWave {
    fn new(pos: f32, vel: f32, acc: f32) -> SlideWave {
        SlideWave { pos, vel, half_acc: 0.5 * acc, time: 0.0 }
    }
}

impl WaveGen for SlideWave {
    fn next(&mut self, step: f32) -> Option<Sample> {
        let time = self.time;
        self.time += step;
        Some(self.pos + (self.vel + self.half_acc * time) * time)
    }

    fn reset(&mut self) {
        self.time = 0.0;
    }

    fn as_wave(&self) -> Wave {
        Wave::new(Box::new(self.clone()))
    }
}

// ========================================================================= //

/// A waveform consisting of the sum of two other waveforms.
#[derive(Clone)]
struct Sum {
    wave1: Wave,
    wave2: Wave,
}

impl WaveGen for Sum {
    fn next(&mut self, step: f32) -> Option<Sample> {
        match self.wave1.next(step) {
            Some(value1) => Some(match self.wave2.next(step) {
                Some(value2) => value1 + value2,
                None => value1,
            }),
            None => self.wave2.next(step),
        }
    }

    fn reset(&mut self) {
        self.wave1.reset();
        self.wave2.reset();
    }

    fn as_wave(&self) -> Wave {
        Wave::new(Box::new(self.clone()))
    }
}

// ========================================================================= //

/// A variable-frequency, variable-duty triangle wave, with an amplitude of 1.
#[derive(Clone)]
struct TriangleWave {
    freq: Wave,
    duty: Wave,
    phase: f32,
}

impl TriangleWave {
    fn new(freq: Wave, duty: Wave) -> TriangleWave {
        TriangleWave { freq, duty, phase: 0.0 }
    }
}

impl WaveGen for TriangleWave {
    fn next(&mut self, step: f32) -> Option<Sample> {
        let freq = match self.freq.next(step) {
            Some(freq) => freq,
            None => return None,
        };
        let duty = match self.duty.next(step) {
            Some(duty) => duty,
            None => return None,
        };
        let phase = self.phase;
        self.phase = (self.phase + freq * step) % 1.0;
        Some(if phase < duty {
            2.0 * phase / duty - 1.0
        } else {
            1.0 - 2.0 * (phase - duty) / (1.0 - duty)
        })
    }

    fn reset(&mut self) {
        self.freq.reset();
        self.duty.reset();
        self.phase = 0.0;
    }

    fn as_wave(&self) -> Wave {
        Wave::new(Box::new(self.clone()))
    }
}

// ========================================================================= //

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::SQRT_2;

    macro_rules! assert_approx {
        ($left:expr, $right:expr) => {{
            match (&($left), &($right)) {
                (left_val, right_val) => {
                    if (*left_val - *right_val).abs() > 1e-6 {
                        panic!(
                            "assertion failed: `(left ~= right)` \
                                (left: `{:?}`, right: `{:?}`)",
                            left_val, right_val
                        )
                    }
                }
            }
        }};
    }

    #[test]
    fn sine_wave() {
        let step = 1.0 / 22050.0;
        let mut wave = Wave::sine(2756.25);
        assert_approx!(0.0, wave.next(step).unwrap());
        assert_approx!(0.5 * SQRT_2, wave.next(step).unwrap());
        assert_approx!(1.0, wave.next(step).unwrap());
        assert_approx!(0.5 * SQRT_2, wave.next(step).unwrap());
        assert_approx!(0.0, wave.next(step).unwrap());
        assert_approx!(-0.5 * SQRT_2, wave.next(step).unwrap());
        assert_approx!(-1.0, wave.next(step).unwrap());
        assert_approx!(-0.5 * SQRT_2, wave.next(step).unwrap());
        assert_approx!(0.0, wave.next(step).unwrap());
        wave.reset();
        assert_approx!(0.0, wave.next(step).unwrap());
        assert_approx!(0.5 * SQRT_2, wave.next(step).unwrap());
        assert_approx!(1.0, wave.next(step).unwrap());
    }

    #[test]
    fn triangle_wave() {
        let step = 0.1;
        let mut wave = Wave::triangle(1.0, 0.8);
        assert_approx!(-1.0, wave.next(step).unwrap());
        assert_approx!(-0.75, wave.next(step).unwrap());
        assert_approx!(-0.5, wave.next(step).unwrap());
        assert_approx!(-0.25, wave.next(step).unwrap());
        assert_approx!(0.0, wave.next(step).unwrap());
        assert_approx!(0.25, wave.next(step).unwrap());
        assert_approx!(0.5, wave.next(step).unwrap());
        assert_approx!(0.75, wave.next(step).unwrap());
        assert_approx!(1.0, wave.next(step).unwrap());
        assert_approx!(0.0, wave.next(step).unwrap());
        assert_approx!(-1.0, wave.next(step).unwrap());
        assert_approx!(-0.75, wave.next(step).unwrap());
    }

    #[test]
    fn wave_clone() {
        let step = 1.0 / 500.0;
        let mut wave1 = Wave::noise(440.0);
        let mut wave2 = wave1.clone();
        for _ in 0..1000 {
            assert_approx!(
                wave1.next(step).unwrap(),
                wave2.next(step).unwrap()
            );
        }
    }

    #[test]
    fn wave_sum() {
        let mut wave = Wave::sine(0.25) + 1.5;
        assert_approx!(1.5, wave.next(1.0).unwrap());
        assert_approx!(2.5, wave.next(1.0).unwrap());
        assert_approx!(1.5, wave.next(1.0).unwrap());
        assert_approx!(0.5, wave.next(1.0).unwrap());
    }
}

// ========================================================================= //
