# LlamaLogAnalyzer MLX Edition - System Architecture

This document describes the architecture of the LlamaLogAnalyzer MLX Edition project, providing an overview of its components, interactions, and design principles.

## System Overview

LlamaLogAnalyzer MLX Edition is a hybrid Rust/Python application that leverages the strengths of both languages:

- **Rust Core**: High-performance, memory-safe concurrent processing for log parsing, statistical analysis, and visualization
- **Python MLX**: Machine learning capabilities for anomaly detection, optimized for Apple Silicon using the MLX framework

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        User Interfaces                          │
│                                                                 │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────────┐  │
│  │ CLI Interface│    │    TUI       │    │ JSON/CSV Output  │  │
│  └──────────────┘    └──────────────┘    └──────────────────┘  │
└───────────────────────────────┬─────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Core Components                         │
│                                                                 │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────────┐  │
│  │ Log Parsers  │───▶│ Analyzers    │───▶│ Visualizers      │  │
│  └──────────────┘    └──────────────┘    └──────────────────┘  │
│         ▲                   │                     ▲            │
└─────────┼───────────────────┼─────────────────────┼────────────┘
          │                   │                     │
          │                   ▼                     │
┌─────────┼───────────────────────────────────────┬─┼────────────┐
│         │       ML Components (Python/MLX)      │ │            │
│  ┌──────┴──────┐    ┌──────────────┐    ┌───────┴─────┐        │
│  │ Data        │    │ Anomaly      │    │ ML Results  │        │
│  │ Preprocessor│───▶│ Detector     │───▶│ Processor   │        │
│  └─────────────┘    └──────────────┘    └─────────────┘        │
└─────────────────────────────────────────────────────────────────┘
```

## Component Details

### 1. User Interfaces

#### CLI Interface (`src/cli/app.rs`)
- Command-line argument parsing and handling
- Provides text-based output for analysis results
- Configurable output formats (text, JSON, CSV)

#### TUI Interface (`src/cli/tui.rs`)
- Interactive terminal user interface
- Real-time visualizations and dashboards
- User-friendly navigation and filtering

#### Output Formatters
- JSON/CSV export functionality
- Structured output for integration with other tools

### 2. Core Components (Rust)

#### Log Parsers (`src/parsers/`)
- Parse different log formats (standard, JSON, syslog)
- Extract structured data from unstructured logs
- Handle different timestamp formats and timezones

#### Analyzers (`src/analyzer/`)
- Statistical analysis of log data
- Pattern recognition
- Time-series analysis
- Frequency analysis

#### Visualizers (`src/visualizers/`)
- Terminal-based visualizations
- Charts and graphs for log data
- Interactive data exploration

### 3. ML Components (Python/MLX)

#### Data Preprocessor
- Converts Rust log data to Python/MLX format
- Feature extraction and normalization
- Prepares data for ML models

#### Anomaly Detector
- ML-powered anomaly detection
- Autoencoder model for unsupervised learning
- Pattern-based anomaly detection
- Time-series anomaly detection

#### ML Results Processor
- Processes ML model outputs
- Converts results back to Rust format
- Enriches results with confidence scores and metadata

## Data Flow

1. **Log Ingestion**: Log files are read and parsed by the Rust parsers
2. **Initial Analysis**: Basic statistical analysis is performed in Rust
3. **ML Processing**: Data is passed to Python for ML-based analysis
4. **Visualization**: Results are visualized through CLI or TUI
5. **Output**: Analysis results can be exported in various formats

## Integration Between Rust and Python

The Rust and Python components communicate through the PyO3 library, which provides Rust bindings to the Python interpreter:

- Rust log entries are converted to Python dictionaries
- Python ML analysis results are converted back to Rust structures
- The MLX framework is used exclusively in the Python side for ML acceleration

## Design Principles

1. **Performance**: Concurrency and parallelism through Rust's Rayon
2. **Safety**: Memory safety through Rust's ownership model
3. **Modularity**: Well-defined components with clear responsibilities
4. **Extensibility**: Plugin-based architecture for parsers and analyzers
5. **Hardware Acceleration**: MLX optimization for Apple Silicon

## Error Handling

- Comprehensive error propagation using Rust's `Result` and `anyhow` 
- Graceful fallback to basic analysis when ML is unavailable
- Detailed logging for troubleshooting

## Configuration

- Environment-based configuration
- Command-line options for runtime behavior
- Configuration file support for persistent settings

## Future Enhancements

1. **Distributed Processing**: Support for distributed log analysis
2. **Real-time Monitoring**: Live log file watching
3. **Advanced Visualization**: More sophisticated charts and dashboards
4. **Cloud Integration**: AWS/GCP/Azure log analysis
5. **Custom ML Models**: User-defined models for specialized log analysis

## Development Workflow

For details on the development workflow, testing, and contribution guidelines, please refer to the [CONTRIBUTING.md](../../CONTRIBUTING.md) file. 