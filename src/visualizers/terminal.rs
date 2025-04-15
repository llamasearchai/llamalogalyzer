//! Terminal-based visualizations
use crate::parsers::LogEntry;
use crate::analyzer::statistics::LogStatistics;
use colored::*;
use std::collections::HashMap;

/// Print a simple bar chart to the terminal
pub fn print_level_distribution(stats: &LogStatistics) {
    println!("\n{}", "Log Level Distribution:".bold());
    
    let max_count = stats.level_counts.values().max().cloned().unwrap_or(0);
    let width = 50; // Max bar width
    
    let mut sorted_levels: Vec<_> = stats.level_counts.iter().collect();
    sorted_levels.sort_by_key(|(_, &count)| std::cmp::Reverse(count));
    
    for (level, &count) in sorted_levels {
        let percentage = (count as f64 / stats.total_entries as f64) * 100.0;
        let bar_length = (count as f64 / max_count as f64 * width as f64) as usize;
        
        let bar = "█".repeat(bar_length);
        let bar_colored = match level.as_str() {
            "ERROR" => bar.red(),
            "WARNING" => bar.yellow(),
            "INFO" => bar.green(),
            "DEBUG" => bar.blue(),
            _ => bar.normal(),
        };
        
        println!("{:>8}: {} {:>5} ({:.1}%)", level, bar_colored, count, percentage);
    }
}

/// Print timeline of log events
pub fn print_timeline(entries: &[LogEntry]) {
    let timestamped_entries: Vec<_> = entries
        .iter()
        .filter_map(|e| e.timestamp.map(|ts| (e, ts)))
        .collect();
        
    if timestamped_entries.is_empty() {
        println!("No timestamped entries to display timeline.");
        return;
    }
    
    println!("\n{}", "Timeline of Events:".bold());
    
    // Group by hour
    let mut hour_groups: HashMap<String, usize> = HashMap::new();
    
    for (_, ts) in &timestamped_entries {
        let hour_key = ts.format("%Y-%m-%d %H:00").to_string();
        *hour_groups.entry(hour_key).or_insert(0) += 1;
    }
    
    let max_count = hour_groups.values().max().cloned().unwrap_or(0);
    let width = 40; // Max bar width
    
    let mut sorted_hours: Vec<_> = hour_groups.iter().collect();
    sorted_hours.sort_by(|a, b| a.0.cmp(b.0));
    
    for (hour, &count) in sorted_hours {
        let bar_length = (count as f64 / max_count as f64 * width as f64) as usize;
        let bar = "█".repeat(bar_length);
        
        println!("{}: {} {}", hour, bar.cyan(), count);
    }
} 