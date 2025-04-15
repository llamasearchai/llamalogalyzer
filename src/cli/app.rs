//! Main CLI application logic
use clap::{Parser};
use std::path::PathBuf;
use anyhow::{Result, Context};
use crate::analyzer::statistics::compute_statistics;
use crate::parsers::LogEntry;
use crate::ml::anomaly::AnomalyDetector;
use serde_json;

/// LlamaLogAnalyzer MLX Edition CLI
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    /// Path to log file
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    
    /// Run in interactive mode
    #[arg(short, long)]
    pub interactive: bool,
    
    /// Output format (text, json, csv)
    #[arg(short, long, default_value = "text")]
    pub output: String,
}

pub fn run_cli() -> Result<()> {
    let args = CliArgs::parse();
    
    if let Some(file) = args.file {
        println!("Analyzing log file: {}", file.display());
        
        // Parse log file
        let entries: Vec<LogEntry> = futures::executor::block_on(
            crate::parsers::parse_log_file(&file)
        ).context("Failed to parse log file")?;
        
        if entries.is_empty() {
            println!("No log entries found in the file.");
            return Ok(());
        }
        
        // Compute statistics
        let stats = compute_statistics(&entries);
        
        // Detect anomalies
        let detector = AnomalyDetector::new(0.7);
        let anomalies = detector.detect_anomalies(&entries);
        
        // Output results based on format
        match args.output.as_str() {
            "json" => {
                // Create a structure for JSON output
                let output = serde_json::json!({
                    "statistics": {
                        "total_entries": stats.total_entries,
                        "level_counts": stats.level_counts,
                        "time_span": stats.time_span.map(|ts| {
                            serde_json::json!({
                                "first_entry": ts.first_entry.to_rfc3339(),
                                "last_entry": ts.last_entry.to_rfc3339(),
                                "duration_seconds": ts.duration_seconds
                            })
                        }),
                        "average_time_diff": stats.average_time_diff
                    },
                    "anomalies": anomalies
                });
                
                println!("{}", serde_json::to_string_pretty(&output)?);
            },
            "csv" => {
                println!("category,key,value");
                println!("statistics,total_entries,{}", stats.total_entries);
                
                for (level, count) in stats.level_counts {
                    println!("level_count,{},{}", level, count);
                }
                
                if let Some(ts) = stats.time_span {
                    println!("time_span,first_entry,{}", ts.first_entry.to_rfc3339());
                    println!("time_span,last_entry,{}", ts.last_entry.to_rfc3339());
                    println!("time_span,duration_seconds,{}", ts.duration_seconds);
                }
                
                if let Some(avg) = stats.average_time_diff {
                    println!("statistics,average_time_diff,{}", avg);
                }
                
                for (i, anomaly) in anomalies.iter().enumerate() {
                    println!("anomaly,{},\"{}\"", i, anomaly.description.replace("\"", "\"\""));
                }
            },
            _ => { // Default to text
                println!("\n=== Log Analysis Results ===");
                println!("Total log entries: {}", stats.total_entries);
                
                println!("\nLog Level Distribution:");
                for (level, count) in stats.level_counts {
                    println!("  {}: {}", level, count);
                }
                
                if let Some(time_span) = stats.time_span {
                    println!("\nTime Span:");
                    println!("  First Entry: {}", time_span.first_entry.format("%Y-%m-%d %H:%M:%S"));
                    println!("  Last Entry: {}", time_span.last_entry.format("%Y-%m-%d %H:%M:%S"));
                    println!("  Duration: {} seconds", time_span.duration_seconds);
                }
                
                if let Some(avg_diff) = stats.average_time_diff {
                    println!("\nAverage Time Between Entries: {:.2} seconds", avg_diff);
                }
                
                if !anomalies.is_empty() {
                    println!("\nDetected Anomalies:");
                    for (i, anomaly) in anomalies.iter().enumerate() {
                        println!("  {}. {} (Confidence: {:.1}%, Severity: {}/5)", 
                            i+1, 
                            anomaly.description, 
                            anomaly.confidence * 100.0,
                            anomaly.severity
                        );
                    }
                } else {
                    println!("\nNo anomalies detected.");
                }
            }
        }
    } else if args.interactive {
        // Run TUI
        crate::cli::tui::TUIApp::new(None).run()?;
    } else {
        println!("No log file specified. Use --help for usage.");
    }
    
    Ok(())
} 