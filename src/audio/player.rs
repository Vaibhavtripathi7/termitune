use log::info;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct AudioPlayer {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: Sink,
    current_track: Option<PathBuf>,
    is_paused: Arc<AtomicBool>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to get audio output: {}", e))?;
        let sink = Sink::try_new(&stream_handle)?;

        Ok(Self {
            _stream: stream,
            _stream_handle: stream_handle,
            sink,
            current_track: None,
            is_paused: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn play(&mut self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        info!("Playing: {:?}", path);

        self.stop();

        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;

        self.sink.append(source);
        self.current_track = Some(path.clone());

        Ok(())
    }

    pub fn pause(&self) {
        self.sink.pause();
        self.is_paused.store(true, Ordering::SeqCst);
    }

    pub fn resume(&self) {
        self.sink.play();
        self.is_paused.store(false, Ordering::SeqCst);
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn is_playing(&self) -> bool {
        !self.sink.is_paused() && !self.sink.empty()
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    #[allow(dead_code)]
    pub fn current_track(&self) -> Option<&PathBuf> {
        self.current_track.as_ref()
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume.clamp(0.0, 1.0));
    }

    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    #[allow(dead_code)]
    pub fn seek_forward(&mut self, _duration: Duration) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    #[allow(dead_code)]
    pub fn seek_backward(&mut self, _duration: Duration) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    #[allow(dead_code)]
    pub fn empty(&self) -> bool {
        self.sink.empty()
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new().expect("Failed to create audio player")
    }
}
