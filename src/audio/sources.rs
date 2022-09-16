use std::time::Duration;

use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Normal;
use rodio::{Sample, Source};

use crate::audio::{AdjustableSource, SAMPLE_RATE};

#[derive(Copy, Clone, Debug, PartialEq)]
/// A square wave source, with adjustable frequency. Toggles from 1 to -1.
pub struct SquareWave {
    phase: f32,
    frequency: f32,
}

impl SquareWave {
    /// Create a new square wave source with the given frequency.
    pub fn new(frequency: f32) -> SquareWave {
        SquareWave {
            phase: 0.0,
            frequency,
        }
    }
}

impl Source for SquareWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for SquareWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.phase < 0.5 { 1.0 } else { -1.0 };
        self.phase = (self.phase + self.frequency / SAMPLE_RATE as f32) % 1.0;
        Some(result)
    }
}

impl AdjustableSource for SquareWave {
    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// A sawtooth wave source, with adjustable frequency. Linearly increases, then drops down instantly.
pub struct SawtoothWave {
    phase: f32,
    frequency: f32,
}

impl SawtoothWave {
    /// Create a new sawtooth wave source with the given frequency.
    pub fn new(frequency: f32) -> SawtoothWave {
        SawtoothWave {
            phase: 0.0,
            frequency,
        }
    }
}

impl Source for SawtoothWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for SawtoothWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.phase * 2.0 - 1.0;
        self.phase = (self.phase + self.frequency / SAMPLE_RATE as f32) % 1.0;
        Some(result)
    }
}

impl AdjustableSource for SawtoothWave {
    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// A triangle wave source, with adjustable frequency. Linearly increases, then linearly decreases.
pub struct TriangleWave {
    phase: f32,
    frequency: f32,
}

impl TriangleWave {
    /// Create a new triangle wave source with the given frequency.
    pub fn new(frequency: f32) -> TriangleWave {
        TriangleWave {
            phase: 0.0,
            frequency,
        }
    }
}

impl Source for TriangleWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for TriangleWave {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.phase < 0.5 {
            self.phase * 4.0 - 1.0
        } else {
            -(self.phase * 4.0 - 3.0)
        };
        self.phase = (self.phase + self.frequency / SAMPLE_RATE as f32) % 1.0;
        Some(result)
    }
}

impl AdjustableSource for TriangleWave {
    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// A sine wave source, with adjustable frequency. Generates a sine wave with the given frequency.
pub struct SineWave {
    phase: f32,
    frequency: f32,
}

impl SineWave {
    /// Create a new sine wave source with the given frequency.
    pub fn new(frequency: f32) -> SineWave {
        SineWave {
            phase: 0.0,
            frequency,
        }
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for SineWave {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.phase * 2.0 * std::f32::consts::PI).sin();
        self.phase = (self.phase + self.frequency / SAMPLE_RATE as f32) % 1.0;
        Some(result)
    }
}

impl AdjustableSource for SineWave {
    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// A noise source. Generates a random number between -1 and 1 with a normal distribution.
pub struct WhiteNoise;

impl WhiteNoise {
    /// Create a new noise source.
    pub fn new() -> WhiteNoise {
        WhiteNoise
    }
}

impl AdjustableSource for WhiteNoise {
    fn set_frequency(&mut self, _frequency: f32) {}
}

impl Source for WhiteNoise {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for WhiteNoise {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let result = Normal::new(0.0, 1.0).unwrap().sample(&mut thread_rng());
        Some(result)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// A semi-triangle source, with adjustable frequency. Linearly goes from -1 to 1, then down to 0, back to 1, then down to -1.
pub struct SemiTriangle {
    phase: f32,
    frequency: f32,
}

impl SemiTriangle {
    /// Create a new semi-triangle wave source with the given frequency.
    pub fn new(frequency: f32) -> SemiTriangle {
        SemiTriangle {
            phase: 0.0,
            frequency,
        }
    }
}

impl Source for SemiTriangle {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for SemiTriangle {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.phase <= 0.25 {
            self.phase * 8.0 - 1.0
        } else if self.phase <= 0.5 {
            1.0 - (self.phase - 0.25) * 4.0
        } else if self.phase <= 0.75 {
            (self.phase - 0.5) * 4.0
        } else {
            1.0 - ((self.phase - 0.75) * 8.0)
        };
        self.phase = (self.phase + self.frequency / SAMPLE_RATE as f32) % 1.0;
        Some(result)
    }
}

impl AdjustableSource for SemiTriangle {
    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// A semi-sine source, with adjustable frequency. Goes up from -1 to 1, then back down, but only with the positive half of a sine wave mapped to it.
pub struct SemiSine {
    phase: f32,
    frequency: f32,
}

impl SemiSine {
    /// Create a new semi-sine wave source with the given frequency.
    pub fn new(frequency: f32) -> SemiSine {
        SemiSine {
            phase: 0.0,
            frequency,
        }
    }
}

impl Source for SemiSine {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for SemiSine {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.phase * std::f32::consts::PI).sin() * 2.0 - 1.0;
        self.phase = (self.phase + self.frequency / SAMPLE_RATE as f32) % 1.0;
        Some(result)
    }
}

impl AdjustableSource for SemiSine {
    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// A stepping square source, with adjustable frequency. Goes up and back down in steps of 1 (-1, 0, 1, 0, etc)
pub struct StepSquare {
    phase: f32,
    frequency: f32,
}

impl StepSquare {
    /// Create a new stepping square wave source with the given frequency.
    pub fn new(frequency: f32) -> StepSquare {
        StepSquare {
            phase: 0.0,
            frequency,
        }
    }
}

impl Source for StepSquare {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for StepSquare {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.phase <= 0.25 {
            -1.0
        } else if self.phase <= 0.5 {
            0.0
        } else if self.phase <= 0.75 {
            1.0
        } else {
            0.0
        };
        self.phase = (self.phase + self.frequency / SAMPLE_RATE as f32) % 1.0;
        Some(result)
    }
}

impl AdjustableSource for StepSquare {
    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}

#[cfg(test)]
mod tests {
    use rodio::{OutputStream, Sink};

    use crate::audio::sources::*;

    #[test]
    fn all_test() {
        sine_test();
        square_test();
        sawtooth_test();
        triangle_test();
        white_noise_test();
    }

    #[test]
    fn sine_test() {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).ok().unwrap();
        let source = SineWave::new(220.0);
        sink.set_volume(0.2);
        sink.append(source);
        std::thread::sleep(Duration::from_secs(2));
    }

    #[test]
    fn square_test() {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).ok().unwrap();
        let source = SquareWave::new(220.0);
        sink.set_volume(0.2);
        sink.append(source);
        std::thread::sleep(Duration::from_secs(2));
    }

    #[test]
    fn sawtooth_test() {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).ok().unwrap();
        let source = SawtoothWave::new(220.0);
        sink.set_volume(0.2);
        sink.append(source);
        std::thread::sleep(Duration::from_secs(2));
    }

    #[test]
    fn triangle_test() {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).ok().unwrap();
        let source = TriangleWave::new(220.0);
        sink.set_volume(0.2);
        sink.append(source);
        std::thread::sleep(Duration::from_secs(2));
    }

    #[test]
    fn semitriangle_test() {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).ok().unwrap();
        let source = SemiTriangle::new(220.0);
        sink.set_volume(0.2);
        sink.append(source);
        std::thread::sleep(Duration::from_secs(2));
    }

    #[test]
    fn semisine_test() {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).ok().unwrap();
        let source = SemiSine::new(220.0);
        sink.set_volume(0.2);
        sink.append(source);
        std::thread::sleep(Duration::from_secs(2));
    }

    #[test]
    fn stepsquare_test() {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).ok().unwrap();
        let source = StepSquare::new(220.0);
        sink.set_volume(0.2);
        sink.append(source);
        std::thread::sleep(Duration::from_secs(2));
    }

    #[test]
    fn white_noise_test() {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).ok().unwrap();
        let source = WhiteNoise;
        sink.set_volume(0.2);
        sink.append(source);
        std::thread::sleep(Duration::from_secs(2));
    }
}
