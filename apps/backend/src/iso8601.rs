/// Convert seconds to ISO8601 duration string (PT#S format)
pub fn seconds_to_duration(seconds: u64) -> String {
    format!("PT{}S", seconds)
}

/// Parse ISO8601 duration string (PT#S format) to seconds
pub fn duration_to_seconds(duration: &str) -> Option<u64> {
    // Simple parser for PT#S format
    if duration.starts_with("PT") && duration.ends_with('S') {
        let seconds_str = &duration[2..duration.len() - 1];
        seconds_str.parse::<u64>().ok()
    } else {
        None
    }
}
