//! Standard log parser
//! Expected format: "YYYY-MM-DD HH:MM:SS [LEVEL] Message"
use chrono::NaiveDateTime;
use crate::parsers::LogEntry;

pub fn parse_line(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.splitn(3, ' ').collect();
    if parts.len() < 3 { return None; }
    
    let ts_str = format!("{} {}", parts[0], parts[1]);
    let timestamp = NaiveDateTime::parse_from_str(&ts_str, "%Y-%m-%d %H:%M:%S").ok().map(|t| t.and_utc());
    
    let remainder = parts[2];
    let level_start = remainder.find('[')?;
    let level_end = remainder.find(']')?;
    
    let level = remainder[level_start+1..level_end].to_string();
    let message = remainder[level_end+2..].to_string();
    
    Some(LogEntry { 
        timestamp, 
        level, 
        message, 
        source: None 
    })
}
