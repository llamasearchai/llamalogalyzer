# Advanced Anomaly Detection Example

This example demonstrates how to use the advanced anomaly detection capabilities of LlamaLogAnalyzer MLX Edition to identify unusual patterns in log data.

## Overview

The example performs the following steps:

1. Loads a training set of "normal" log data
2. Trains the ML model on this normal data
3. Processes a test set of logs with potential anomalies
4. Analyzes and categorizes the detected anomalies
5. Outputs both textual descriptions and visualizations

## Prerequisites

- LlamaLogAnalyzer MLX Edition installed and configured
- Python MLX environment set up
- Sample log files (normal and anomalous)
- Rust toolchain

## Code Example

```rust
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use llamaloganalyzer_mlx::{
    ml::anomaly::{AnomalyDetector, AnomalyType, AnomalyResult},
    parsers::{LogEntry, parse_log_file},
    utils::logger,
    visualizers::charts,
};
use log::{info, warn};
use std::collections::HashMap;
use std::path::Path;
use tokio::runtime::Runtime;

/// Advanced anomaly detection example
fn main() -> Result<()> {
    // Initialize logging
    logger::init()?;
    info!("Starting advanced anomaly detection example");
    
    // Create tokio runtime for async operations
    let rt = Runtime::new()?;
    
    // Step 1: Load normal log data for training
    info!("Loading normal log data for training...");
    let normal_logs_path = "examples/data/normal_logs.log";
    let normal_entries = rt.block_on(parse_log_file(normal_logs_path))?;
    info!("Loaded {} normal log entries for training", normal_entries.len());
    
    // Step 2: Create and train the detector
    info!("Creating anomaly detector...");
    let detector = AnomalyDetector::new(0.7, true);
    
    info!("Training anomaly detector on normal log data...");
    detector.train(&normal_entries)?;
    info!("Training completed");
    
    // Step 3: Load test log data with potential anomalies
    info!("Loading test log data...");
    let test_logs_path = "examples/data/test_logs.log";
    let test_entries = rt.block_on(parse_log_file(test_logs_path))?;
    info!("Loaded {} test log entries", test_entries.len());
    
    // Step 4: Detect anomalies
    info!("Detecting anomalies in test data...");
    let anomalies = detector.detect_anomalies(&test_entries)?;
    info!("Detection complete - found {} anomalies", anomalies.len());
    
    // Step 5: Analyze and categorize anomalies
    analyze_anomalies(&anomalies, &test_entries);
    
    // Step 6: Visualize results (if running with --features=visualization)
    #[cfg(feature = "visualization")]
    {
        info!("Generating visualizations...");
        charts::create_anomaly_timeline(&test_entries, &anomalies, "anomaly_timeline.png")?;
        charts::create_anomaly_distribution(&anomalies, "anomaly_types.png")?;
        info!("Visualizations saved to current directory");
    }
    
    info!("Example completed successfully");
    Ok(())
}

/// Analyze and categorize detected anomalies
fn analyze_anomalies(anomalies: &[AnomalyResult], entries: &[LogEntry]) {
    if anomalies.is_empty() {
        println!("No anomalies detected in the log data.");
        return;
    }
    
    // Group anomalies by type
    let mut anomaly_types = HashMap::new();
    for anomaly in anomalies {
        let anomaly_name = format!("{:?}", anomaly.anomaly_type);
        *anomaly_types.entry(anomaly_name).or_insert(0) += 1;
    }
    
    // Print anomaly summary
    println!("\n=== Anomaly Detection Results ===");
    println!("Total anomalies detected: {}", anomalies.len());
    println!("\nAnomalies by type:");
    for (anomaly_type, count) in &anomaly_types {
        println!("  {}: {}", anomaly_type, count);
    }
    
    // Print detailed anomaly information
    println!("\nTop anomalies (by confidence):");
    let mut sorted_anomalies = anomalies.to_vec();
    sorted_anomalies.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
    
    for (i, anomaly) in sorted_anomalies.iter().take(5).enumerate() {
        println!("\n{}. {} (confidence: {:.2})", 
            i + 1,
            format!("{:?}", anomaly.anomaly_type),
            anomaly.confidence);
        println!("   Description: {}", anomaly.description);
        println!("   Severity: {}/10", anomaly.severity);
        println!("   Time: {}", anomaly.timestamp.format("%Y-%m-%d %H:%M:%S"));
        
        println!("   Related entries:");
        for &idx in anomaly.related_entries.iter().take(3) {
            if idx < entries.len() {
                let entry = &entries[idx];
                println!("     - [{}] {}", entry.level, entry.message);
            }
        }
        
        if anomaly.related_entries.len() > 3 {
            println!("     - ... and {} more", anomaly.related_entries.len() - 3);
        }
    }
    
    if sorted_anomalies.len() > 5 {
        println!("\n... and {} more anomalies", sorted_anomalies.len() - 5);
    }
    
    // Provide some recommendations
    println!("\n=== Recommendations ===");
    if anomaly_types.contains_key("FrequencyAnomaly") {
        println!("- Investigate log frequency spikes - they may indicate service issues");
    }
    if anomaly_types.contains_key("ContentAnomaly") {
        println!("- Review unusual log messages, especially those with high confidence scores");
    }
    if anomaly_types.contains_key("TimePatternAnomaly") {
        println!("- Check for disruptions in regular service patterns");
    }
    if anomaly_types.contains_key("RepeatingError") {
        println!("- Address repeating error patterns to improve system stability");
    }
}
```

## Creating Test Data

To create suitable test data for this example, you can generate normal logs and logs with anomalies:

### Normal Logs (examples/data/normal_logs.log)

Generate logs with a consistent pattern of informational messages, occasional warnings, and very few errors:

```bash
# Generate normal logs for training
mkdir -p examples/data
for i in {1..1000}; do
  severity="INFO"
  if (( i % 20 == 0 )); then severity="WARNING"; fi
  if (( i % 100 == 0 )); then severity="ERROR"; fi
  
  timestamp=$(date -v+${i}M +"%Y-%m-%d %H:%M:%S")
  echo "$timestamp [$severity] Normal application operation, iteration $i" >> examples/data/normal_logs.log
done
```

### Test Logs with Anomalies (examples/data/test_logs.log)

Generate logs that include normal patterns but also contain anomalies:

```bash
# Generate test logs with anomalies
for i in {1..1000}; do
  severity="INFO"
  if (( i % 20 == 0 )); then severity="WARNING"; fi
  if (( i % 200 == 0 )); then severity="ERROR"; fi
  
  timestamp=$(date -v+${i}M +"%Y-%m-%d %H:%M:%S")
  
  # Normal logs
  if (( i < 400 || i > 500 )) && (( i < 700 || i > 750 )); then
    echo "$timestamp [$severity] Normal application operation, iteration $i" >> examples/data/test_logs.log
  # Frequency anomaly - burst of errors
  elif (( i >= 400 && i <= 500 )); then
    echo "$timestamp [ERROR] Connection timeout: unable to reach database" >> examples/data/test_logs.log
  # Content anomaly - unusual messages
  elif (( i >= 700 && i <= 750 )); then
    echo "$timestamp [WARNING] Unexpected memory consumption spike: $(( RANDOM % 1000 + 8000 ))MB" >> examples/data/test_logs.log
  fi
done
```

## Running the Example

To run this example:

1. Ensure your environment is set up:
   ```bash
   cd llamaloganalyzer-mlx
   ./setup_mlx_env.sh
   ```

2. Compile and run the example:
   ```bash
   cargo run --release --example advanced_anomaly_detection
   ```

3. With visualization (requires the `visualization` feature):
   ```bash
   cargo run --release --features=visualization --example advanced_anomaly_detection
   ```

## Expected Output

The example should produce output similar to:

```
=== Anomaly Detection Results ===
Total anomalies detected: 32

Anomalies by type:
  FrequencyAnomaly: 23
  ContentAnomaly: 9

Top anomalies (by confidence):
1. FrequencyAnomaly (confidence: 0.98)
   Description: Unusual error rate: 100x normal frequency
   Severity: 8/10
   Time: 2023-05-15 12:43:21
   Related entries:
     - [ERROR] Connection timeout: unable to reach database
     - [ERROR] Connection timeout: unable to reach database
     - [ERROR] Connection timeout: unable to reach database
     - ... and 18 more

2. ContentAnomaly (confidence: 0.92)
   Description: Unusual log message pattern detected
   Severity: 7/10
   Time: 2023-05-15 16:32:45
   Related entries:
     - [WARNING] Unexpected memory consumption spike: 8435MB
     - [WARNING] Unexpected memory consumption spike: 8912MB
     - [WARNING] Unexpected memory consumption spike: 8211MB
     - ... and 6 more

... and 30 more anomalies

=== Recommendations ===
- Investigate log frequency spikes - they may indicate service issues
- Review unusual log messages, especially those with high confidence scores
```

## Visualization Examples

If you run with the `visualization` feature, the example generates:

1. **anomaly_timeline.png**: A timeline showing when anomalies occurred
2. **anomaly_types.png**: A pie chart showing the distribution of anomaly types

## Next Steps

After running this example, you might want to:

1. Experiment with different threshold values to adjust sensitivity
2. Try different types of log data with various anomaly patterns
3. Implement a custom visualization for the anomalies
4. Create a more sophisticated anomaly detector by extending the Python MLX code 