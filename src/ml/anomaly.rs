//! Anomaly detection using MLX through Python integration
use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::parsers::LogEntry;
use std::collections::HashMap;
use std::process::Command;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyType {
    FrequencyAnomaly,
    ContentAnomaly,
    TimePatternAnomaly,
    SecurityAnomaly,
    SystemAnomaly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyResult {
    pub anomaly_type: AnomalyType,
    pub confidence: f64,
    pub description: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub related_entries: Vec<usize>,
    pub severity: u8,
}

pub struct AnomalyDetector {
    pub threshold: f64,
    python_available: bool,
}

impl AnomalyDetector {
    pub fn new(threshold: f64) -> Self {
        // Check if Python environment is available
        let python_available = Path::new("venv").exists() && 
            Command::new("python3")
                .arg("-c")
                .arg("import mlx.core")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
                
        Self { threshold, python_available }
    }
    
    pub fn detect_anomalies(&self, entries: &[LogEntry]) -> Vec<AnomalyResult> {
        if entries.len() < 10 { return vec![]; }
        
        // Calculate log level frequencies
        let mut level_counts: HashMap<String, i32> = HashMap::new();
        for entry in entries {
            *level_counts.entry(entry.level.clone()).or_insert(0) += 1;
        }
        
        // Look for unusual patterns
        let mut anomalies = vec![];
        
        // Sample anomaly detection (in production, this would call the Python code)
        let error_count = level_counts.get("ERROR").unwrap_or(&0);
        if *error_count > 3 {
            // Find related entries (indices of ERROR logs)
            let mut related_entries = vec![];
            for (i, entry) in entries.iter().enumerate() {
                if entry.level == "ERROR" {
                    related_entries.push(i);
                }
            }
            
            anomalies.push(AnomalyResult {
                anomaly_type: AnomalyType::FrequencyAnomaly,
                confidence: 0.85,
                description: format!("Unusual spike in ERROR logs detected ({} occurrences)", error_count),
                timestamp: Utc::now(),
                related_entries,
                severity: 4,
            });
        }
        
        anomalies
    }
} 