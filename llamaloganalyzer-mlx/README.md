# LlamaLogAnalyzer MLX Edition

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![License](https://img.shields.io/badge/license-MIT-blue)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange)
![MLX](https://img.shields.io/badge/MLX-0.3.0+-purple)

<p align="center">
  <img src="assets/llamaloganalyzer-logo.png" alt="LlamaLogAnalyzer Logo" width="300" onerror="this.style.display='none'"/>
</p>

LlamaLogAnalyzer MLX Edition is a high-performance, ML-accelerated log analysis tool that leverages Rust's concurrent processing capabilities combined with Apple's MLX framework for advanced log analysis, pattern detection, and anomaly identification.

## 🚀 Features

### Core Capabilities
- **High-Performance Concurrent Processing** - Utilizes Rust's Rayon for parallel log processing, handling millions of log entries efficiently
- **ML-Powered Anomaly Detection** - Leverages Apple's MLX framework on Apple Silicon M-series chips for real-time anomaly detection
- **Interactive TUI Dashboard** - Sleek terminal-based user interface with real-time visualizations using Ratatui
- **Comprehensive Log Analysis** - Extracts metrics, patterns, and insights from structured and unstructured logs

### Log Format Support
- ✅ Standard timestamp-based logs (`YYYY-MM-DD HH:MM:SS [LEVEL] Message`)
- ✅ JSON logs with automatic schema detection
- ✅ Syslog format
- ✅ Custom formats via regex pattern matching

### Analysis Capabilities
- 📊 Time-based pattern recognition and anomaly detection
- 📈 Statistical analysis of log frequencies, patterns, and trends
- 🔍 Message clustering and categorization
- ⚠️ Critical event identification and alerting potential

## 📋 Architecture

LlamaLogAnalyzer MLX Edition follows a hybrid architecture:

```
┌─────────────────────────┐      ┌─────────────────────┐
│   Rust Core Components  │◄────►│ Python MLX Pipeline │
├─────────────────────────┤      ├─────────────────────┤
│ ┌─────────┐ ┌─────────┐ │      │ ┌─────────────────┐ │
│ │ Parsers │ │ Analyzer│ │      │ │ Anomaly Models  │ │
│ └─────────┘ └─────────┘ │      │ └─────────────────┘ │
│ ┌─────────┐ ┌─────────┐ │      │ ┌─────────────────┐ │
│ │  Stats  │ │  TUI    │ │      │ │ Pattern Detector│ │
│ └─────────┘ └─────────┘ │      │ └─────────────────┘ │
└─────────────────────────┘      └─────────────────────┘
```

- **Rust Core**: High-performance concurrent log parsing, analysis, and visualization
- **Python MLX Integration**: ML-powered anomaly detection optimized for Apple Silicon

## ⚡ Performance Benchmarks

| Log Size      | Processing Time | Memory Usage |
|---------------|----------------|--------------|
| 10K entries   | 0.3s           | ~15MB        |
| 100K entries  | 2.5s           | ~45MB        |
| 1M entries    | 18s            | ~250MB       |

*Benchmarks performed on MacBook Pro M3 Max with 64GB RAM*

## 🛠️ Installation

### Prerequisites
- Rust 1.75 or newer
- Python 3.10 or newer
- Apple Silicon Mac (for MLX acceleration)

### Setup

1. Clone this repository
```bash
git clone https://github.com/yourusername/llamaloganalyzer-mlx.git
cd llamaloganalyzer-mlx
```

2. Set up the Python MLX environment:
```bash
./setup_mlx_env.sh
```

3. Build the project:
```bash
cargo build --release
```

## 📊 Usage

### Command-Line Interface

Analyze a specific log file:
```bash
./main.sh --file /path/to/logfile.log
```

Run with interactive TUI dashboard:
```bash
./main.sh --interactive
```

Run with sample logs:
```bash
./main.sh --sample
```

For all available options:
```bash
./main.sh --help
```

### Interactive TUI Navigation

| Key          | Action                     |
|--------------|----------------------------|
| Tab          | Switch between views       |
| Arrow keys   | Navigate within views      |
| Enter        | Select item                |
| q            | Quit application           |

### Example Output

```
═════════════════════════════════════════════════════════
       🦙 LLAMALOGANALYZER MLX EDITION v2.0.0 🦙       
═════════════════════════════════════════════════════════
Total log entries: 60
ERROR: 5
INFO: 31
WARNING: 6
DEBUG: 17
CRITICAL: 1

--- Time-based Analysis ---
Log Timespan: 2023-05-15 08:12:34 -> 2023-05-15 13:00:10
Total Duration: 4h 47m 36s
Average Time Between Entries: 292.47s
Maximum Time Between Entries: 2385.00s
Minimum Time Between Entries: 1.00s

--- Pattern Analysis ---
Top Message Patterns:
  - "Hourly" appears 10 times
  - "Memory" appears 6 times
  - "CPU" appears 5 times
  - "Disk" appears 5 times
  - "Database" appears 3 times
```

## 🧠 ML Capabilities

The MLX integration enables several advanced features:

- **Anomaly Detection**: Identifies unusual patterns in log frequency, content, or timing
- **Log Clustering**: Groups similar log messages for better visualization and analysis
- **Temporal Pattern Recognition**: Detects cyclical patterns and deviations from normal behavior

## 📈 Future Roadmap

- [ ] Real-time log monitoring via file watchers
- [ ] Enhanced visualization with more chart types
- [ ] Cloud integration for persistent storage and analysis
- [ ] Collaborative analysis with shared dashboards
- [ ] Extended ML models for predictive analytics

## 🤝 Contributing

Contributions are welcome! Here's how you can contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please run tests and ensure code quality with:
```bash
cargo test
cargo clippy
```

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgements

- [Apple MLX Team](https://github.com/ml-explore/mlx) for the excellent ML framework
- [Ratatui](https://github.com/ratatui-org/ratatui) for the TUI framework
- All contributors and early adopters of this project

---

<p align="center">Built with ❤️ by <a href="https://github.com/yourusername">Your Name</a></p>
