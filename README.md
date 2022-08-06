# RetroCore
A library for easily creating windows and audio sources for fantasy consoles

# API
## Canvas
Create a new canvas with
```rs
RetroCanvas::new((1000, 600), (320, 240), "Test");
```
The parameters are, in order, the real window size, the simulated window size (i.e. the size of the console's window), and the title.

A RetroCanvas derefs to a `Canvas<Window>` from SDL2, which means you can use all their methods (and should; that's how you draw to the window). Please refer to their documentation for more information. You must draw to the console window directly, with regular SDL2 code.
The only exception is clearing the screen; clearing the screen also clears the backdrop of the window, which means if you clear with a color different from the background (black, by default), you will overwrite that. Instead, use `clear_simulated`

## Audio
Audio is created by creating a `Channels` struct, which contains a list of different channels. Each channel is an audio source of its own, with adjustable frequency. When you create a Channels struct, you also receive the hook. The hook permits you to control the frequencies and volumes of all the different channels.

The following adjustable sources are provided, but you can create more by implementing `AdjustableSource`:
- Square wave
- Sawtooth wave
- Triangle wave
- Sine wave
- White noise

The audio can then be more easily played than with rodio, using `ChannelPlayback::new(channels);`. If ChannelPlayback is dropped, the audio stops playing. ChannelPlayback also derefs to a Sink, for general control over the channels.

Example code:
```rs
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
```

