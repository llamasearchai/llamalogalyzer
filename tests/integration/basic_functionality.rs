use llamaloganalyzer_mlx::analyzer::statistics::compute_statistics;
use llamaloganalyzer_mlx::parsers::LogEntry;
use chrono::{Utc, TimeZone};

#[test]
fn test_end_to_end_log_stats() {
    // Create some test log entries
    let log_entries = vec![
        LogEntry {
            timestamp: Some(Utc.with_ymd_and_hms(2023, 12, 1, 8, 0, 0).unwrap()),
            level: "INFO".to_string(),
            message: "Test message 1".to_string(),
            source: None,
        },
        LogEntry {
            timestamp: Some(Utc.with_ymd_and_hms(2023, 12, 1, 8, 0, 30).unwrap()),
            level: "ERROR".to_string(),
            message: "Test error".to_string(),
            source: None,
        },
        LogEntry {
            timestamp: Some(Utc.with_ymd_and_hms(2023, 12, 1, 8, 1, 0).unwrap()),
            level: "INFO".to_string(),
            message: "Test message 2".to_string(),
            source: None,
        },
    ];
    
    // Compute statistics
    let stats = compute_statistics(&log_entries);
    
    // Verify results
    assert_eq!(stats.total_entries, 3);
    assert_eq!(stats.level_counts.get("INFO"), Some(&2));
    assert_eq!(stats.level_counts.get("ERROR"), Some(&1));
    
    // Check time span
    let time_span = stats.time_span.unwrap();
    assert_eq!(time_span.duration_seconds, 60);
    
    // Check average time difference
    assert!(stats.average_time_diff.is_some());
    assert_eq!(stats.average_time_diff.unwrap(), 30.0);
} 