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
    
    // Calculate log level counts
    let mut level_counts = HashMap::new();
    for entry in entries {
        *level_counts.entry(entry.level.clone()).or_insert(0) += 1;
    }
    
    // Calculate time span
    let time_span = if !entries.is_empty() {
        // Find first and last entries with timestamps
        let timestamps: Vec<_> = entries.iter()
            .filter_map(|e| e.timestamp)
            .collect();
            
        if !timestamps.is_empty() {
            let first = timestamps.first().unwrap();
            let last = timestamps.last().unwrap();
            
            Some(TimeSpan {
                first_entry: *first,
                last_entry: *last,
                duration_seconds: (last - first).num_seconds(),
            })
        } else {
            None
        }
    } else { 
        None 
    };
    
    // Calculate average time difference between consecutive log entries
    let average_time_diff = if total_entries > 1 {
        let mut total_diff = 0.0;
        let mut count = 0;
        
        // Extract entries with timestamps and sort them
        let mut timestamped_entries: Vec<_> = entries.iter()
            .filter_map(|e| e.timestamp.map(|ts| (e, ts)))
            .collect();
            
        timestamped_entries.sort_by_key(|(_, ts)| *ts);
        
        // Calculate differences
        for i in 1..timestamped_entries.len() {
            let (_, prev_ts) = timestamped_entries[i-1];
            let (_, curr_ts) = timestamped_entries[i];
            
            total_diff += (curr_ts - prev_ts).num_seconds() as f64;
            count += 1;
        }
        
        if count > 0 {
            Some(total_diff / count as f64)
        } else {
            None
        }
    } else { 
        None 
    };
    
    LogStatistics { 
        total_entries, 
        level_counts, 
        time_span, 
        average_time_diff 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    
    #[test]
    fn test_compute_statistics() {
        let entries = vec![
            LogEntry {
                timestamp: Some(Utc.with_ymd_and_hms(2023, 12, 1, 10, 0, 0).unwrap()),
                level: "INFO".to_string(),
                message: "Test 1".to_string(),
                source: None,
            },
            LogEntry {
                timestamp: Some(Utc.with_ymd_and_hms(2023, 12, 1, 10, 1, 0).unwrap()),
                level: "ERROR".to_string(),
                message: "Test 2".to_string(),
                source: None,
            },
            LogEntry {
                timestamp: Some(Utc.with_ymd_and_hms(2023, 12, 1, 10, 2, 30).unwrap()),
                level: "INFO".to_string(),
                message: "Test 3".to_string(),
                source: None,
            },
        ];
        
        let stats = compute_statistics(&entries);
        
        assert_eq!(stats.total_entries, 3);
        assert_eq!(*stats.level_counts.get("INFO").unwrap(), 2);
        assert_eq!(*stats.level_counts.get("ERROR").unwrap(), 1);
        
        let time_span = stats.time_span.unwrap();
        assert_eq!(time_span.duration_seconds, 150); // 2m30s = 150s
        
        assert!(stats.average_time_diff.is_some());
        assert_eq!(stats.average_time_diff.unwrap(), 75.0); // (60 + 90) / 2 = 75
    }
} 