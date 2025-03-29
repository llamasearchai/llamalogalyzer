//! Main CLI application logic
use clap::{Parser};
use std::path::PathBuf;
use anyhow::Result;
use tokio::runtime::Runtime;
use crate::analyzer::standard::StandardAnalyzer;
use chrono::Duration;
use crate::parsers::parse_log_file;

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
        
        // Create a new Tokio runtime
        let rt = Runtime::new()?;
        
        // Load and analyze the file
        let entries = rt.block_on(parse_log_file(&file))?;
        
        // Use our enhanced StandardAnalyzer
        let results = StandardAnalyzer::analyze(&entries);
        
        // Display results with enhanced output
        println!("Total log entries: {}", results.total_entries);
        
        // Display log level counts
        for (level, count) in &results.level_counts {
            println!("{}: {}", level, count);
        }
        
        // Display time-based metrics
        if results.time_metrics.total_duration_seconds > 0 {
            println!("\n--- Time-based Analysis ---");
            println!("Log Timespan: {} -> {}", 
                results.time_metrics.start_time.format("%Y-%m-%d %H:%M:%S"),
                results.time_metrics.end_time.format("%Y-%m-%d %H:%M:%S"));
            
            let duration = Duration::seconds(results.time_metrics.total_duration_seconds);
            let hours = duration.num_hours();
            let minutes = duration.num_minutes() % 60;
            let seconds = duration.num_seconds() % 60;
            
            println!("Total Duration: {}h {}m {}s", hours, minutes, seconds);
            println!("Average Time Between Entries: {:.2}s", results.time_metrics.avg_time_diff_seconds);
            println!("Maximum Time Between Entries: {:.2}s", results.time_metrics.max_time_diff_seconds);
            println!("Minimum Time Between Entries: {:.2}s", results.time_metrics.min_time_diff_seconds);
        }
        
        // Display pattern analysis
        if !results.pattern_analysis.top_patterns.is_empty() || 
           !results.pattern_analysis.top_prefixes.is_empty() {
            println!("\n--- Pattern Analysis ---");
            
            if !results.pattern_analysis.top_patterns.is_empty() {
                println!("Top Message Patterns:");
                for (pattern, count) in &results.pattern_analysis.top_patterns {
                    println!("  - \"{}\" appears {} times", pattern, count);
                }
            }
            
            if !results.pattern_analysis.top_prefixes.is_empty() {
                println!("\nCommon Message Prefixes:");
                for (prefix, count) in &results.pattern_analysis.top_prefixes {
                    println!("  - \"{}\" appears {} times", prefix, count);
                }
            }
        }
    } else if args.interactive {
        crate::cli::tui::run_tui()?;
    } else {
        println!("No log file specified. Use --help for usage.");
    }
    
    Ok(())
}
