use lofty::file::AudioFile;
use lofty::prelude::*;
use log::info;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TrackMetadata {
    pub title: String,
    pub artist: String,
    #[allow(dead_code)]
    pub album: String,
    pub duration: Duration,
    pub path: PathBuf,
}

impl TrackMetadata {
    pub fn from_path(path: &PathBuf) -> Option<Self> {
        match lofty::read_from_path(path) {
            Ok(tagged_file) => {
                let properties = tagged_file.properties();
                let duration = properties.duration();

                let tag = tagged_file
                    .primary_tag()
                    .or_else(|| tagged_file.first_tag());

                let title = tag
                    .and_then(|t| t.title().map(|s| s.to_string()))
                    .unwrap_or_else(|| {
                        path.file_stem()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_default()
                    });

                let artist = tag
                    .and_then(|t| t.artist().map(|s| s.to_string()))
                    .unwrap_or_else(|| "Unknown Artist".to_string());

                let album = tag
                    .and_then(|t| t.album().map(|s| s.to_string()))
                    .unwrap_or_else(|| "Unknown Album".to_string());

                Some(Self {
                    title,
                    artist,
                    album,
                    duration,
                    path: path.clone(),
                })
            }
            Err(e) => {
                info!("Could not read metadata for {:?}: {}", path, e);
                Some(Self {
                    title: path.file_stem()?.to_string_lossy().to_string(),
                    artist: "Unknown Artist".to_string(),
                    album: "Unknown Album".to_string(),
                    duration: Duration::ZERO,
                    path: path.clone(),
                })
            }
        }
    }

    pub fn format_duration(&self) -> String {
        let secs = self.duration.as_secs();
        let minutes = secs / 60;
        let seconds = secs % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
}

pub fn is_audio_file(path: &PathBuf) -> bool {
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        matches!(
            ext.as_str(),
            "mp3" | "flac" | "wav" | "ogg" | "m4a" | "aac" | "wma"
        )
    } else {
        false
    }
}
