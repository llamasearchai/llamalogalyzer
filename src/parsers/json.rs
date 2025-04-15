//! JSON log parser module
use serde_json::Value;
use chrono::{DateTime, Utc, NaiveDateTime};
use crate::parsers::LogEntry;

pub fn parse_json_line(line: &str) -> Option<LogEntry> {
    let parsed: Value = match serde_json::from_str(line) {
        Ok(json) => json,
        Err(_) => return None,
    };
    
    // Extract timestamp
    let timestamp = if let Some(ts_str) = parsed.get("timestamp").and_then(|t| t.as_str()) {
        NaiveDateTime::parse_from_str(ts_str, "%Y-%m-%d %H:%M:%S")
            .ok()
            .map(|t| t.and_utc())
    } else {
        None
    };
    
    // Extract log level
    let level = parsed.get("level")
        .and_then(|l| l.as_str())
        .unwrap_or("UNKNOWN")
        .to_string();
    
    // Extract message
    let message = parsed.get("message")
        .and_then(|m| m.as_str())
        .unwrap_or("")
        .to_string();
    
    // Extract source (optional)
    let source = parsed.get("source")
        .and_then(|s| s.as_str())
        .map(|s| s.to_string());
    
    Some(LogEntry {
        timestamp,
        level,
        message,
        source,
    })
} 