use std::path::Path;
use tracing::error;

use symphonia::core::{
    formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint,
};

pub struct AudioMetadata {
    pub duration: u64,
}

pub fn get_audio_metadata(path: &Path) -> Option<AudioMetadata> {
    let duration = get_audio_duration(path);
    if duration.is_none() {
        return None;
    }
    Some(AudioMetadata {
        duration: duration.unwrap(),
    })
}

fn get_audio_duration(path: &Path) -> Option<u64> {
    // Open the media source
    let file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to open file {:?}: {}", path, e);
            return None;
        }
    };

    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    // Create a hint to help the format registry guess the format
    let mut hint = Hint::new();
    if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(extension);
    }

    // Probe the media source
    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();

    let probed =
        match symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts) {
            Ok(p) => p,
            Err(e) => {
                error!("Failed to probe file {:?}: {}", path, e);
                return None;
            }
        };

    let format = probed.format;

    // Find the default track
    let track = match format.default_track() {
        Some(t) => t,
        None => {
            error!("No default track found in {:?}", path);
            return None;
        }
    };

    // Calculate duration from the track
    let duration_secs = if let Some(time_base) = track.codec_params.time_base {
        if let Some(n_frames) = track.codec_params.n_frames {
            let duration = time_base.calc_time(n_frames);
            duration.seconds
        } else {
            error!("No frame count available for {:?}", path);
            return None;
        }
    } else {
        error!("No time base available for {:?}", path);
        return None;
    };

    Some(duration_secs)
}
