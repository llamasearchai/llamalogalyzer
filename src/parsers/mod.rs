//! Log parsers for various formats
pub mod standard;
pub mod json;
pub use standard::parse_line;
pub use json::parse_json_line;

use anyhow::Result;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// Log entry structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: Option<DateTime<Utc>>,
    pub level: String,
    pub message: String,
    pub source: Option<String>,
}

pub async fn parse_log_file<P: AsRef<Path>>(path: P) -> Result<Vec<LogEntry>> {
    let file = File::open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut entries = Vec::new();
    
    while let Some(line) = lines.next_line().await? {
        // Try standard format first
        if let Some(entry) = standard::parse_line(&line) {
            entries.push(entry);
            continue;
        }
        
        // Try JSON format
        if let Some(entry) = json::parse_json_line(&line) {
            entries.push(entry);
            continue;
        }
        
        // Other formats could be added here
    }
    
    Ok(entries)
} 