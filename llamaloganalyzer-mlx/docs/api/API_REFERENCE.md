# LlamaLogAnalyzer MLX Edition API Reference

This document provides a reference for the main APIs provided by the LlamaLogAnalyzer MLX Edition library.

## Table of Contents

- [Core Types](#core-types)
- [Parsers API](#parsers-api)
- [Analyzer API](#analyzer-api)
- [ML Integration API](#ml-integration-api)
- [Visualization API](#visualization-api)
- [CLI API](#cli-api)

## Core Types

### LogEntry

The core data structure representing a parsed log entry.

```rust
pub struct LogEntry {
    /// The timestamp of the log entry (optional)
    pub timestamp: Option<DateTime<Utc>>,
    
    /// The log level (INFO, WARNING, ERROR, etc.)
    pub level: String,
    
    /// The log message content
    pub message: String,
    
    /// The source of the log (optional)
    pub source: Option<String>,
}
```

### AnalysisResults

The results of log analysis.

```rust
pub struct AnalysisResults {
    pub total_entries: usize,
    pub level_counts: HashMap<String, usize>,
    pub time_metrics: TimeMetrics,
    pub pattern_analysis: PatternAnalysis,
}

pub struct TimeMetrics {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub total_duration_seconds: i64,
    pub avg_time_diff_seconds: f64,
    pub max_time_diff_seconds: f64,
    pub min_time_diff_seconds: f64,
}

pub struct PatternAnalysis {
    pub top_patterns: Vec<(String, usize)>,
    pub top_prefixes: Vec<(String, usize)>,
}
```

### AnomalyResult

The results of anomaly detection.

```rust
pub enum AnomalyType {
    FrequencyAnomaly,
    ContentAnomaly,
    TimePatternAnomaly,
    WindowAnomaly,
    RepeatingError,
    Custom(String),
}

pub struct AnomalyResult {
    pub anomaly_type: AnomalyType,
    pub confidence: f64,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub related_entries: Vec<usize>,
    pub severity: u8,
}
```

## Parsers API

### parse_log_file

Parses a log file into a vector of `LogEntry` objects.

```rust
pub async fn parse_log_file(path: impl AsRef<Path>) -> Result<Vec<LogEntry>> {
    // ...
}
```

Example:
```rust
let entries = parse_log_file("path/to/logfile.log").await?;
```

### parse_line

Parses a single log line into a `LogEntry`.

```rust
pub fn parse_line(line: &str) -> Option<LogEntry> {
    // ...
}
```

Example:
```rust
let log_entry = parse_line("2023-05-15 08:12:34 [INFO] Application started");
```

### Custom Parsers

Custom parsers can be implemented by creating a module that provides a `parse_line` function:

```rust
pub mod custom_parser {
    use crate::parsers::LogEntry;
    
    pub fn parse_line(line: &str) -> Option<LogEntry> {
        // Custom parsing logic
    }
}
```

## Analyzer API

### StandardAnalyzer

Performs standard analysis on log entries.

```rust
pub struct StandardAnalyzer;

impl StandardAnalyzer {
    pub fn analyze(entries: &[LogEntry]) -> AnalysisResults {
        // ...
    }
}
```

Example:
```rust
let results = StandardAnalyzer::analyze(&entries);
println!("Total entries: {}", results.total_entries);
```

## ML Integration API

### AnomalyDetector

Detects anomalies in log entries using MLX.

```rust
pub struct AnomalyDetector {
    // ...
}

impl AnomalyDetector {
    pub fn new(threshold: f64, verbose: bool) -> Self {
        // ...
    }
    
    pub fn detect_anomalies(&self, entries: &[LogEntry]) -> Result<Vec<AnomalyResult>> {
        // ...
    }
    
    pub fn train(&self, normal_entries: &[LogEntry]) -> Result<()> {
        // ...
    }
}
```

Example:
```rust
let detector = AnomalyDetector::new(0.7, true);
detector.train(&normal_log_entries)?;
let anomalies = detector.detect_anomalies(&log_entries)?;

for anomaly in anomalies {
    println!("Detected {} with confidence {:.2}", 
        anomaly.anomaly_type, anomaly.confidence);
}
```

## Visualization API

### create_log_histogram

Creates a histogram of log occurrences over time.

```rust
pub fn create_log_histogram(
    entries: &[LogEntry], 
    output_path: &Path,
    title: &str
) -> Result<()> {
    // ...
}
```

Example:
```rust
create_log_histogram(&entries, Path::new("histogram.png"), "Log Frequency")?;
```

### create_level_distribution

Creates a chart showing the distribution of log levels.

```rust
pub fn create_level_distribution(
    results: &AnalysisResults,
    output_path: &Path
) -> Result<()> {
    // ...
}
```

Example:
```rust
create_level_distribution(&results, Path::new("levels.png"))?;
```

## CLI API

### CliArgs

Command-line arguments for the application.

```rust
#[derive(Parser, Debug)]
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
```

### run_cli

Runs the command-line interface.

```rust
pub fn run_cli() -> Result<()> {
    // ...
}
```

Example:
```rust
fn main() -> Result<()> {
    llamaloganalyzer_mlx::init()?;
    run_cli()?;
    Ok(())
}
```

### run_tui

Runs the terminal user interface.

```rust
pub fn run_tui() -> Result<()> {
    // ...
}
```

Example:
```rust
if args.interactive {
    run_tui()?;
} else {
    // Process file...
}
```

## Complete Example

```rust
use anyhow::Result;
use llamaloganalyzer_mlx::{
    parsers::parse_log_file,
    analyzer::standard::StandardAnalyzer,
    ml::anomaly::AnomalyDetector,
};
use std::path::Path;
use tokio::runtime::Runtime;

fn main() -> Result<()> {
    // Initialize the library
    llamaloganalyzer_mlx::init()?;
    
    // Create a runtime for async operations
    let rt = Runtime::new()?;
    
    // Parse log file
    let entries = rt.block_on(parse_log_file("logs/application.log"))?;
    
    // Perform standard analysis
    let results = StandardAnalyzer::analyze(&entries);
    
    // Print basic stats
    println!("Total log entries: {}", results.total_entries);
    for (level, count) in &results.level_counts {
        println!("{}: {}", level, count);
    }
    
    // Detect anomalies
    let detector = AnomalyDetector::new(0.7, false);
    let anomalies = detector.detect_anomalies(&entries)?;
    
    // Print anomalies
    println!("\nDetected {} anomalies:", anomalies.len());
    for anomaly in anomalies {
        println!("- {} (confidence: {:.2}): {}", 
            format!("{:?}", anomaly.anomaly_type), 
            anomaly.confidence,
            anomaly.description);
    }
    
    Ok(())
}
```

## API Stability

APIs marked as `pub` in the crate root are considered stable and follow semantic versioning. Internal APIs (not re-exported from the crate root) may change between minor versions.

For more detailed API documentation, refer to the Rust documentation generated with `cargo doc --open`. 