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
pub struct Channels {
    sources : Vec<Arc<Mutex<dyn AdjustableSource<Item = f32> + Send>>>,
    volume : Vec<Arc<Mutex<f32>>>,
}

pub struct ChannelsBuilder {
    sources : Vec<Arc<Mutex<dyn AdjustableSource<Item = f32> + Send>>>,
}

impl ChannelsBuilder {
    pub fn new() -> Self {
        Self {
            sources : Vec::new(),
        }
    }
    pub fn add_source<T>(mut self, source: T) -> Self
        where
            T: AdjustableSource<Item = f32> + Send + 'static,
    {
        self.sources.push(Arc::new(Mutex::new(source)) as _);
        self
    }

    pub fn add_source_raw<T>(mut self, source: Arc<Mutex<T>>) -> Self
        where
            T: AdjustableSource<Item = f32> + Send + 'static,
    {
        self.sources.push(source);
        self
    }

    pub fn build(self) -> (Channels, ChannelHook) {
        Channels::new(self.sources)
    }
}

impl Channels {
    fn new(sources: Vec<Arc<Mutex<dyn AdjustableSource<Item = f32> + Send>>>) -> (Self, ChannelHook) {
        let volumes : Vec<Arc<Mutex<f32>>> = (0..sources.len()).map(|_| Arc::new(Mutex::new(0.2))).collect();
        for i in &sources {
            let j = i.lock().unwrap();
            if j.total_duration().is_some() || j.current_frame_len().is_some() {
                panic!("Sources can't have a limited duration, and cannot have a finite frame len due to library limitations! Please contact the author with your use case if you cannot work around it.");
            }
            if j.channels() != 1 {
                panic!("Sources can't have more than one channel! Please contact the author with your use case if you cannot work around it.");
            }
        }
        (Channels {
            sources : sources.clone(),
            volume : volumes.clone(),
        }, ChannelHook {
            sources,
            volume: volumes
        })
    }
}

impl Iterator for Channels {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = 0.0;
        for (i, source) in self.sources.iter_mut().enumerate() {
            result += source.lock().unwrap().next().unwrap_or(0.0) * *self.volume[i].lock().unwrap();
        }
        Some(result / self.sources.len() as f32)
    }
}

impl Source for Channels {
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
pub struct ChannelHook {
    volume : Vec<Arc<Mutex<f32>>>,
    sources : Vec<Arc<Mutex<dyn AdjustableSource<Item = f32> + Send>>>,
}

impl ChannelHook {
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
    pub fn new(channels : Channels) -> Self {
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
    use super::*;

    #[test]
    fn test_channels() {
        let (channels, mut hook) = ChannelsBuilder::new()
            .add_source(sources::SineWave::new(220.0))
            .add_source(sources::SquareWave::new(220.0))
            .build();
        let _channel_playback = ChannelPlayback::new(channels);
        hook.set_volume(0, 0.5);
        hook.set_volume(1, 0.1);
        std::thread::sleep(Duration::from_secs(2));
        hook.set_frequency(1, 440.0);
        std::thread::sleep(Duration::from_secs(2));
        hook.set_frequency(0, 440.0);
        std::thread::sleep(Duration::from_secs(2));
        hook.set_volume(1, 0.0);
        std::thread::sleep(Duration::from_secs(2));
    }
}