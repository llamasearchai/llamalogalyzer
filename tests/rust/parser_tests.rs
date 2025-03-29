use llamaloganalyzer_mlx::parsers::{self, LogEntry};

#[test]
fn test_standard_log_parser() {
    let line = "2023-12-01 08:00:01 [INFO] System startup completed";
    let entry = parsers::standard::parse_line(line);
    assert!(entry.is_some());
    
    let entry = entry.unwrap();
    assert_eq!(entry.level, "INFO");
    assert_eq!(entry.message, "System startup completed");
    assert!(entry.timestamp.is_some());
}

#[test]
fn test_json_log_parser() {
    let line = r#"{"timestamp": "2023-12-01 08:00:01", "level": "INFO", "message": "System startup"}"#;
    let entry = parsers::json::parse_json_line(line);
    assert!(entry.is_some());
    
    let entry = entry.unwrap();
    assert_eq!(entry.level, "INFO");
    assert_eq!(entry.message, "System startup");
    assert!(entry.timestamp.is_some());
}

#[test]
fn test_invalid_log_format() {
    let line = "This is not a valid log entry";
    let entry = parsers::standard::parse_line(line);
    assert!(entry.is_none());
    
    let entry = parsers::json::parse_json_line(line);
    assert!(entry.is_none());
} 