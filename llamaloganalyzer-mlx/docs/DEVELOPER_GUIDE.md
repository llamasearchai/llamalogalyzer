# LlamaLogAnalyzer MLX Edition - Developer Guide

This guide provides detailed information for developers who want to contribute to the LlamaLogAnalyzer MLX Edition project. It covers the project architecture, code organization, and development workflows.

## Project Overview

LlamaLogAnalyzer MLX Edition is a high-performance log analysis tool built with a hybrid architecture:
- **Rust Core**: For high-performance concurrent log parsing, analysis, and visualization
- **Python with MLX**: For ML-powered anomaly detection optimized for Apple Silicon

## Directory Structure

```
llamaloganalyzer-mlx/
├── src/                    # Rust source code
│   ├── bin/                # Binary entry points
│   │   └── main.rs         # Main application entry point
│   ├── analyzer/           # Log analysis modules
│   │   └── standard.rs     # Standard analyzer implementation
│   ├── parsers/            # Log parsers for different formats
│   │   ├── mod.rs          # Parser module definitions
│   │   └── standard.rs     # Standard log format parser
│   ├── ml/                 # Machine learning integration
│   │   └── anomaly.rs      # Anomaly detection with MLX integration
│   ├── cli/                # Command-line interface
│   │   ├── app.rs          # CLI application logic
│   │   └── tui.rs          # Terminal UI implementation
│   ├── visualizers/        # Data visualization
│   ├── utils/              # Utility functions
│   │   └── logger.rs       # Logging utilities
│   └── lib.rs              # Library entry point
├── python/                 # Python code
│   ├── mlx_models/         # MLX model implementations
│   │   └── anomaly_detector.py  # Anomaly detection model
│   └── tests/              # Python tests
├── tests/                  # Rust tests
├── examples/               # Example usage
│   ├── basic_analysis.rs   # Basic log analysis example
│   └── sample.log          # Sample log file for examples
├── assets/                 # Static assets (images, etc.)
├── docs/                   # Documentation
├── scripts/                # Utility scripts
│   └── verify_integration.py  # Python-Rust integration verification
├── .github/workflows/      # GitHub Actions CI/CD
├── Cargo.toml              # Rust project configuration
├── requirements.txt        # Python dependencies
├── setup_mlx_env.sh        # MLX environment setup script
├── LICENSE                 # MIT License
├── README.md               # Project overview
├── CONTRIBUTING.md         # Contribution guidelines
├── CODE_OF_CONDUCT.md      # Code of conduct
└── CHANGELOG.md            # Project changelog
```

## Core Components

### 1. Rust Core

#### Parsers (`src/parsers/`)
The parsers are responsible for reading log files in various formats and converting them into a unified `LogEntry` structure. Currently supported formats:
- Standard timestamp-based logs (`YYYY-MM-DD HH:MM:SS [LEVEL] Message`)
- JSON logs
- Syslog format

#### Analyzers (`src/analyzer/`)
Analyzers process the parsed log entries to extract statistical information and detect patterns. The `StandardAnalyzer` provides:
- Log level distribution
- Time-based metrics
- Message pattern recognition

#### ML Integration (`src/ml/`)
This module provides interfaces between Rust and Python, allowing the use of MLX models from Rust code:
- `anomaly.rs`: Anomaly detection interface

#### CLI (`src/cli/`)
User interface components:
- `app.rs`: Command-line argument handling and application flow
- `tui.rs`: Interactive terminal UI using Ratatui

### 2. Python ML Components

#### Anomaly Detector (`python/mlx_models/anomaly_detector.py`)
The anomaly detector uses MLX to build a neural network model that can identify unusual patterns in logs:
- Frequency-based anomalies (unusual numbers of logs)
- Content-based anomalies (unusual message content)
- Temporal anomalies (unusual timing patterns)

## Development Workflow

### Setting Up the Development Environment

1. Clone the repository
2. Set up the Python MLX environment:
   ```bash
   ./setup_mlx_env.sh
   ```
3. Build the project:
   ```bash
   cargo build
   ```

### Making Changes

#### Changing Rust Code

1. Make your changes in the relevant modules
2. Run tests to ensure functionality:
   ```bash
   cargo test
   ```
3. Check for style issues:
   ```bash
   cargo fmt
   cargo clippy
   ```

#### Changing Python Code

1. Make changes to Python files in the `python/` directory
2. Test your changes:
   ```bash
   cd llamaloganalyzer-mlx
   python -m unittest discover python/tests/
   ```
3. Verify Python-Rust integration:
   ```bash
   ./scripts/verify_integration.py
   ```

## Python-Rust Integration Details

The integration between Rust and Python is handled through PyO3, which allows:
1. Calling Python code from Rust
2. Passing data between Rust and Python

Example from `src/ml/anomaly.rs`:
```rust
// Convert Rust log entries to Python objects
let result = Python::with_gil(|py| -> Result<Vec<AnomalyResult>> {
    // Import Python module
    let mlx_module = PyModule::import(py, "python.mlx_models.anomaly_detector")?;
    
    // Create list of log entries
    let py_entries = self.log_entries_to_py_list(py, entries)?;
    
    // Call Python function
    let kwargs = [("threshold", self.threshold)].into_py_dict(py);
    let py_results = mlx_module.getattr("detect_anomalies")?.call(
        (py_entries,),
        Some(kwargs)
    )?;
    
    // Convert Python results back to Rust
    self.py_results_to_rust(py, py_results)
});
```

### Data Flow

1. Log entries are parsed in Rust
2. When anomaly detection is requested, data is converted to Python dictionaries
3. The MLX model processes the data in Python
4. Results are converted back to Rust structures
5. The Rust application presents the results to the user

## Performance Considerations

- **Memory Management**: Large log files are processed in chunks to manage memory usage
- **Concurrency**: The Rust core uses Rayon for parallel processing
- **MLX Acceleration**: Neural network operations are accelerated on Apple Silicon

## Debugging Tips

### Rust Debugging

Use `RUST_LOG` environment variable to control log levels:
```bash
RUST_LOG=debug cargo run -- --file examples/sample.log
```

### Python Debugging

For Python code, add print statements or use the Python debugger:
```python
import pdb; pdb.set_trace()
```

### Integration Issues

If you encounter issues with the Python-Rust integration:
1. Run the verification script: `./scripts/verify_integration.py -v`
2. Check Python paths and module imports
3. Verify that the MLX environment is correctly set up

## Building Documentation

Generate Rust documentation:
```bash
cargo doc --no-deps --open
```

## Roadmap and Priorities

Current development priorities:
1. Improving anomaly detection accuracy
2. Adding support for more log formats
3. Enhancing visualization capabilities
4. Implementing real-time log monitoring

## Getting Help

If you need assistance:
- Check the existing issues on GitHub
- Join our community chat
- Refer to the [CONTRIBUTING.md](../CONTRIBUTING.md) guide

---

Thank you for contributing to LlamaLogAnalyzer MLX Edition! 