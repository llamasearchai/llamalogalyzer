use chrono::{DateTime, Duration, Utc};
use llamaloganalyzer_mlx::ml::anomaly::{AnomalyDetector, AnomalyType};
use llamaloganalyzer_mlx::parsers::LogEntry;

#[test]
fn test_heuristic_anomaly_detection() {
    // Skip this test in CI environments without Python
    if std::env::var("CI").is_ok() {
        println!("Skipping Python-dependent test in CI environment");
        return;
    }

    // Create a detector
    let detector = AnomalyDetector::new(0.7, true);
    
    // Create a set of log entries with an anomaly pattern (high error rate)
    let entries = generate_test_logs_with_anomaly();
    
    // Run anomaly detection
    let result = detector.detect_anomalies(&entries);
    
    // We expect the function to succeed
    assert!(result.is_ok(), "Anomaly detection failed: {:?}", result.err());
    
    // Get the anomalies
    let anomalies = result.unwrap();
    
    // We expect at least one anomaly to be detected (error frequency)
    assert!(!anomalies.is_empty(), "Expected to detect at least one anomaly");
    
    // Verify at least one of the anomalies is a frequency anomaly
    let has_frequency_anomaly = anomalies.iter().any(|a| {
        matches!(a.anomaly_type, AnomalyType::FrequencyAnomaly)
    });
    
    assert!(has_frequency_anomaly, "Expected to detect a frequency anomaly");
}

// Helper function to generate logs with an anomaly pattern (high error rate)
fn generate_test_logs_with_anomaly() -> Vec<LogEntry> {
    let base_time = Utc::now() - Duration::hours(1);
    let mut entries = Vec::new();
    
    // Add some normal logs
    for i in 0..20 {
        let timestamp = base_time + Duration::minutes(i as i64);
        entries.push(LogEntry {
            timestamp: Some(timestamp),
            level: "INFO".to_string(),
            message: format!("Normal log message {}", i),
            source: Some("test".to_string()),
        });
    }
    
    // Add an error burst (anomaly)
    for i in 0..10 {
        let timestamp = base_time + Duration::minutes((20 + i) as i64);
        entries.push(LogEntry {
            timestamp: Some(timestamp),
            level: "ERROR".to_string(),
            message: format!("Error occurred: Connection timeout #{}", i),
            source: Some("test".to_string()),
        });
    }
    
    // Add some more normal logs
    for i in 0..15 {
        let timestamp = base_time + Duration::minutes((30 + i) as i64);
        entries.push(LogEntry {
            timestamp: Some(timestamp),
            level: "INFO".to_string(),
            message: format!("Back to normal log message {}", i),
            source: Some("test".to_string()),
        });
    }
    
    entries
} 