//! Chart generation utilities
use crate::parsers::LogEntry;
use crate::analyzer::statistics::LogStatistics;

/// Generate data for a pie chart of log levels
pub fn generate_level_pie_chart_data(stats: &LogStatistics) -> Vec<(String, f64)> {
    let total = stats.total_entries as f64;
    stats.level_counts
        .iter()
        .map(|(level, &count)| {
            let percentage = (count as f64 / total) * 100.0;
            (level.clone(), percentage)
        })
        .collect()
}

/// Generate data for a time series chart
pub fn generate_time_series_data(entries: &[LogEntry]) -> Vec<(String, usize)> {
    // Group entries by hour
    let mut hour_counts = std::collections::HashMap::new();
    
    for entry in entries {
        if let Some(ts) = entry.timestamp {
            let hour_key = ts.format("%Y-%m-%d %H:00").to_string();
            *hour_counts.entry(hour_key).or_insert(0) += 1;
        }
    }
    
    let mut result: Vec<_> = hour_counts.into_iter().collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    
    result
} 