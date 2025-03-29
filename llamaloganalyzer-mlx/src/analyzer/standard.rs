use crate::parsers::LogEntry;
use chrono::{DateTime, Utc};
use log::info;
use std::collections::HashMap;

/// Analyzes log entries and provides statistics
pub struct StandardAnalyzer;

impl StandardAnalyzer {
    /// Perform analysis on the provided log entries
    pub fn analyze(entries: &[LogEntry]) -> AnalysisResults {
        info!("Analyzing {} log entries", entries.len());
        
        // Count log levels
        let mut level_counts = HashMap::new();
        for entry in entries {
            *level_counts.entry(entry.level.clone()).or_insert(0) += 1;
        }
        
        // Calculate time-based metrics
        let mut time_metrics = TimeMetrics::default();
        
        // Filter entries with valid timestamps for time-based analysis
        let entries_with_timestamps: Vec<&LogEntry> = entries
            .iter()
            .filter(|entry| entry.timestamp.is_some())
            .collect();
            
        if entries_with_timestamps.len() > 1 {
            // Sort entries by timestamp for time-based analysis
            let mut time_sorted_entries = entries_with_timestamps.clone();
            time_sorted_entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            
            // Calculate time differences
            let mut time_diffs = Vec::new();
            for i in 1..time_sorted_entries.len() {
                if let (Some(current), Some(previous)) = (
                    time_sorted_entries[i].timestamp,
                    time_sorted_entries[i-1].timestamp
                ) {
                    let diff = current.signed_duration_since(previous);
                    time_diffs.push(diff.num_seconds());
                }
            }
            
            if !time_diffs.is_empty() {
                time_metrics.avg_time_diff_seconds = time_diffs.iter().sum::<i64>() as f64 / time_diffs.len() as f64;
                time_metrics.max_time_diff_seconds = *time_diffs.iter().max().unwrap() as f64;
                time_metrics.min_time_diff_seconds = *time_diffs.iter().min().unwrap() as f64;
            }
            
            // Determine time span
            if let (Some(first), Some(last)) = (
                time_sorted_entries.first().unwrap().timestamp,
                time_sorted_entries.last().unwrap().timestamp
            ) {
                time_metrics.start_time = first;
                time_metrics.end_time = last;
                time_metrics.total_duration_seconds = last.signed_duration_since(first).num_seconds();
            }
        }
        
        // Pattern recognition
        let mut message_patterns = HashMap::new();
        let mut common_prefixes = HashMap::new();
        
        for entry in entries {
            // Extract first word of message as pattern indicator
            if let Some(first_word) = entry.message.split_whitespace().next() {
                *message_patterns.entry(first_word.to_string()).or_insert(0) += 1;
            }
            
            // Extract common prefixes (first 3 words)
            let prefix: String = entry.message
                .split_whitespace()
                .take(3)
                .collect::<Vec<_>>()
                .join(" ");
            if !prefix.is_empty() {
                *common_prefixes.entry(prefix).or_insert(0) += 1;
            }
        }
        
        // Find top patterns
        let mut top_patterns = message_patterns.into_iter().collect::<Vec<_>>();
        top_patterns.sort_by(|a, b| b.1.cmp(&a.1));
        let top_patterns = top_patterns.into_iter().take(5).collect();
        
        // Find top prefixes
        let mut top_prefixes = common_prefixes.into_iter().collect::<Vec<_>>();
        top_prefixes.sort_by(|a, b| b.1.cmp(&a.1));
        let top_prefixes = top_prefixes.into_iter().take(5).collect();
        
        // Create pattern analysis
        let pattern_analysis = PatternAnalysis {
            top_patterns,
            top_prefixes,
        };
        
        AnalysisResults {
            total_entries: entries.len(),
            level_counts,
            time_metrics,
            pattern_analysis,
        }
    }
}

/// Results of log analysis
#[derive(Debug)]
pub struct AnalysisResults {
    pub total_entries: usize,
    pub level_counts: HashMap<String, usize>,
    pub time_metrics: TimeMetrics,
    pub pattern_analysis: PatternAnalysis,
}

/// Time-based metrics for log analysis
#[derive(Debug)]
pub struct TimeMetrics {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub total_duration_seconds: i64,
    pub avg_time_diff_seconds: f64,
    pub max_time_diff_seconds: f64,
    pub min_time_diff_seconds: f64,
}

/// Pattern analysis results
#[derive(Debug)]
pub struct PatternAnalysis {
    pub top_patterns: Vec<(String, usize)>,
    pub top_prefixes: Vec<(String, usize)>,
}

impl Default for TimeMetrics {
    fn default() -> Self {
        Self {
            start_time: Utc::now(),
            end_time: Utc::now(),
            total_duration_seconds: 0,
            avg_time_diff_seconds: 0.0,
            max_time_diff_seconds: 0.0,
            min_time_diff_seconds: 0.0,
        }
    }
} 