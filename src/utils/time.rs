//! Time-related utilities
use chrono::{DateTime, Utc, Duration};

/// Format a duration in a human-readable way
pub fn format_duration(seconds: i64) -> String {
    if seconds < 60 {
        return format!("{} seconds", seconds);
    }
    
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    
    if minutes < 60 {
        return format!("{}m {}s", minutes, remaining_seconds);
    }
    
    let hours = minutes / 60;
    let remaining_minutes = minutes % 60;
    
    format!("{}h {}m {}s", hours, remaining_minutes, remaining_seconds)
}

/// Get time difference in a human-readable format
pub fn time_difference(start: DateTime<Utc>, end: DateTime<Utc>) -> String {
    let duration = end.signed_duration_since(start);
    format_duration(duration.num_seconds())
} 