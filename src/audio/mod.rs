pub mod sources;

use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use rodio::{OutputStream, OutputStreamHandle, Sample, Sink};
use rodio::source::Source;

/// A source who's frequency can be adjusted.
pub trait AdjustableSource : Source where
    Self::Item: Sample, {
    fn set_frequency(&mut self, frequency: f32);
}

const SAMPLE_RATE: u32 = 41000;

/// A Source which contains other adjustable sources and plays all of them at once (with adjustable volumes and frequencies).
pub struct Channels<const CHANNEL_COUNT : usize> {
    sources : [Arc<Mutex<Box<dyn AdjustableSource<Item = f32> + Send>>>; CHANNEL_COUNT],
    volume : [Arc<Mutex<f32>>; CHANNEL_COUNT],
}

impl<const CHANNEL_COUNT : usize> Channels<CHANNEL_COUNT> {
    /// Create a new Channels with the given sources. Also returns the hook, which is needed to adjust frequencies after creation.
    pub fn new(sources: [Box<dyn AdjustableSource<Item = f32> + Send>; CHANNEL_COUNT]) -> (Self, ChannelHook<CHANNEL_COUNT>) {
        let volumes = [(); CHANNEL_COUNT].map(|()| Arc::new(Mutex::new(0.2)));
        let sources = sources.map(|x| Arc::new(Mutex::new(x)));
        (Channels {
            sources : sources.clone(),
            volume : volumes.clone(),
        }, ChannelHook {
            volume: volumes,
            sources
        })
    }
}

impl<const CHANNEL_COUNT : usize> Iterator for Channels<CHANNEL_COUNT> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = 0.0;
        for (i, source) in self.sources.iter_mut().enumerate() {
            result += source.lock().unwrap().next()? * *self.volume[i].lock().unwrap();
        }
        Some(result / CHANNEL_COUNT as f32)
    }
}

impl<const CHANNEL_COUNT : usize> Source for Channels<CHANNEL_COUNT> {
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

/// A hook which allows adjusting the volumes and frequencies of the channels after creation.
pub struct ChannelHook<const CHANNEL_COUNT : usize> {
    volume : [Arc<Mutex<f32>>; CHANNEL_COUNT],
    sources : [Arc<Mutex<Box<dyn AdjustableSource<Item = f32> + Send>>>; CHANNEL_COUNT],
}

impl<const CHANNEL_COUNT : usize> ChannelHook<CHANNEL_COUNT> {
    /// Set the frequency of the channel with the given index.
    pub fn set_frequency(&mut self, index : usize, frequency: f32) {
        self.sources[index].lock().unwrap().set_frequency(frequency);
    }

    /// Set the volume of the channel with the given index.
    pub fn set_volume(&mut self, index : usize, volume: f32) {
        *self.volume[index].lock().unwrap() = volume;
    }
}

/// A playback which controls the playing of a Channels. Derefs down to a Sink.
/// DO NOT DROP THIS OR THE CHANNEL WILL STOP PLAYING.
pub struct ChannelPlayback {
    sink : Sink,
    _stream : OutputStream,
    _handle : OutputStreamHandle
}

impl ChannelPlayback {
    /// Create a new ChannelPlayback with the given Channels, and starts playing it.
    pub fn new<const CHANNEL_COUNT : usize>(channels : Channels<CHANNEL_COUNT>) -> Self {
        let (stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).ok().unwrap();
        sink.append(channels);
        sink.play();
        ChannelPlayback {
            sink,
            _stream: stream,
            _handle: handle
        }
    }
}

impl Deref for ChannelPlayback {
    type Target = Sink;

    fn deref(&self) -> &Self::Target {
        &self.sink
    }
}

impl DerefMut for ChannelPlayback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sink
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use rodio::{OutputStream, Sink};
    use super::*;

    #[test]
    fn test_channels() {
        let (channels, mut hook) = Channels::new([
            Box::new(sources::SquareWave::new(220.0)),
            Box::new(sources::SineWave::new(220.0)),
        ]);
        let _channel_playback = ChannelPlayback::new(channels);
        std::thread::sleep(Duration::from_secs(2));
        hook.set_frequency(0, 440.0);
        std::thread::sleep(Duration::from_secs(2));
        hook.set_frequency(1, 440.0);
        std::thread::sleep(Duration::from_secs(2));
        hook.set_volume(0, 0.0);
        std::thread::sleep(Duration::from_secs(2));
    }
}