use anyhow::Result;
use llamaloganalyzer_mlx::analyzer::standard::StandardAnalyzer;
use llamaloganalyzer_mlx::parsers::LogEntry;
use llamaloganalyzer_mlx::utils::logger;
use log::info;
use std::path::Path;
use tokio::runtime::Runtime;

/// This example demonstrates how to perform basic log analysis using LlamaLogAnalyzer
fn main() -> Result<()> {
    // Initialize logger
    logger::init()?;
    
    info!("Starting basic log analysis example");
    
    // Path to log file
    let log_path = "examples/sample.log";
    
    if !Path::new(log_path).exists() {
        eprintln!("Sample log file not found at: {}", log_path);
        eprintln!("Please create it or adjust the path.");
        return Ok(());
    }
    
    // Create a runtime for async operations
    let rt = Runtime::new()?;
    
    // Parse log file
    info!("Parsing log file: {}", log_path);
    let entries = rt.block_on(llamaloganalyzer_mlx::parsers::parse_log_file(log_path))?;
    
    // Print basic stats
    info!("Parsed {} log entries", entries.len());
    
    // Analyze logs using StandardAnalyzer
    info!("Analyzing logs with StandardAnalyzer");
    let results = StandardAnalyzer::analyze(&entries);
    
    // Print analysis results
    println!("\n=== Log Analysis Results ===");
    println!("Total log entries: {}", results.total_entries);
    
    println!("\nLog levels:");
    for (level, count) in &results.level_counts {
        println!("  {}: {}", level, count);
    }
    
    // Print time-based metrics if available
    if results.time_metrics.total_duration_seconds > 0 {
        println!("\nTime-based metrics:");
        println!("  Timespan: {} -> {}", 
            results.time_metrics.start_time.format("%Y-%m-%d %H:%M:%S"),
            results.time_metrics.end_time.format("%Y-%m-%d %H:%M:%S"));
        println!("  Duration: {} seconds", results.time_metrics.total_duration_seconds);
        println!("  Avg time between entries: {:.2}s", results.time_metrics.avg_time_diff_seconds);
    }
    
    // Print pattern analysis
    println!("\nPattern analysis:");
    println!("  Top patterns:");
    for (pattern, count) in &results.pattern_analysis.top_patterns {
        println!("    \"{}\" appears {} times", pattern, count);
    }
    
    println!("\nExample completed successfully!");
    Ok(())
} 