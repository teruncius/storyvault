use std::path::Path;
use tracing::error;

use symphonia::core::{
    formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint,
};

pub struct AudioMetadata {
    pub duration_seconds: Option<u64>,
    pub sample_rate_hz: Option<u32>,
    pub bit_rate_kbps: Option<u64>,
}

pub fn get_audio_metadata(path: &Path) -> Result<AudioMetadata, Box<dyn std::error::Error>> {
    // Open the media source
    let file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to open file {:?}: {}", path, e);
            return Err(e.into());
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
                return Err(e.into());
            }
        };

    let format = probed.format;

    // Find the default track
    let track = match format.default_track() {
        Some(t) => t,
        None => {
            error!("No default track found in {:?}", path);
            return Err("No default track found".into());
        }
    };

    let codec_params = &track.codec_params;

    // Calculate duration from the track
    let duration_seconds = if let Some(time_base) = codec_params.time_base {
        if let Some(n_frames) = codec_params.n_frames {
            let duration = time_base.calc_time(n_frames);
            Some(duration.seconds)
        } else {
            error!("No frame count available for {:?}", path);
            None
        }
    } else {
        error!("No time base available for {:?}", path);
        None
    };

    // Extract sample rate
    let sample_rate_hz = codec_params.sample_rate;

    // Calculate bit rate from file size and duration
    let bit_rate_kbps = if duration_seconds.is_some() {
        std::fs::metadata(path).ok().map(|metadata| {
            let file_size_bits = metadata.len() * 8;
            (file_size_bits / duration_seconds.unwrap()) as u64
        })
    } else {
        None
    };

    Ok(AudioMetadata {
        duration_seconds,
        sample_rate_hz,
        bit_rate_kbps,
    })
}
