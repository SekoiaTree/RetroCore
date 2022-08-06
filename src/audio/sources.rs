use std::time::Duration;
use rand::distributions::Distribution;
use rand_distr::Normal;
use rodio::Source;
use crate::audio::{AdjustableSource, SAMPLE_RATE};

pub struct SquareWave {
    phase: f32,
    frequency: f32,
}

impl SquareWave {
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
        let result = if self.phase < 0.5 {
            1.0
        } else {
            -1.0
        };
        self.phase = (self.phase + self.frequency / SAMPLE_RATE as f32) % 1.0;
        Some(result)
    }
}

impl AdjustableSource for SquareWave {
    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}

pub struct SawtoothWave {
    phase: f32,
    frequency: f32,
}

impl SawtoothWave {
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
        let result = self.phase*2.0-1.0;
        self.phase = (self.phase + self.frequency / SAMPLE_RATE as f32) % 1.0;
        Some(result)
    }
}

impl AdjustableSource for SawtoothWave {
    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}

pub struct TriangleWave {
    phase: f32,
    frequency: f32,
}

impl TriangleWave {
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
            self.phase*4.0-1.0
        } else {
            -(self.phase*4.0-3.0)
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

pub struct SineWave {
    phase: f32,
    frequency: f32,
}

impl SineWave {
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
        let result = (self.phase*2.0*std::f32::consts::PI).sin();
        self.phase = (self.phase + self.frequency / SAMPLE_RATE as f32) % 1.0;
        Some(result)
    }
}

impl AdjustableSource for SineWave {
    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
}

pub struct WhiteNoise;

impl WhiteNoise {
    pub fn new() -> WhiteNoise {
        WhiteNoise
    }
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
        let result = Normal::new(0.0, 1.0).unwrap().sample(&mut rand::thread_rng());
        Some(result)
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
    fn white_noise_test() {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).ok().unwrap();
        let source = WhiteNoise;
        sink.set_volume(0.2);
        sink.append(source);
        std::thread::sleep(Duration::from_secs(2));
    }
}