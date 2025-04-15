//! Log statistics calculation
use std::collections::HashMap;
use crate::parsers::LogEntry;
use chrono::{Utc, DateTime};

#[derive(Debug, Clone)]
pub struct LogStatistics {
    pub total_entries: usize,
    pub level_counts: HashMap<String, usize>,
    pub time_span: Option<TimeSpan>,
    pub average_time_diff: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct TimeSpan {
    pub first_entry: DateTime<Utc>,
    pub last_entry: DateTime<Utc>,
    pub duration_seconds: i64,
}

pub fn compute_statistics(entries: &[LogEntry]) -> LogStatistics {
    let total_entries = entries.len();
    let mut level_counts = HashMap::new();
    
    for entry in entries {
        *level_counts.entry(entry.level.clone()).or_insert(0) += 1;
    }
    
    let time_span = if !entries.is_empty() {
        let first = entries.first().unwrap().timestamp.unwrap_or(Utc::now());
        let last = entries.last().unwrap().timestamp.unwrap_or(Utc::now());
        Some(TimeSpan {
            first_entry: first,
            last_entry: last,
            duration_seconds: (last - first).num_seconds(),
        })
    } else { None };
    
    let average_time_diff = if total_entries > 1 {
        let mut total_diff = 0;
        for w in entries.windows(2) {
            if let (Some(t1), Some(t2)) = (w[0].timestamp, w[1].timestamp) {
                total_diff += (t2 - t1).num_seconds();
            }
        }
        Some(total_diff as f64 / (total_entries - 1) as f64)
    } else { None };
    
    LogStatistics { 
        total_entries, 
        level_counts, 
        time_span, 
        average_time_diff 
    }
}
