# Changelog

All notable changes to the LlamaLogAnalyzer MLX Edition will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
- Enhanced ML-based anomaly detection models
- Support for custom visualization plugins

## [2.0.0] - 2025-02-27
### Added
- MLX integration for Apple Silicon optimization
- Advanced pattern recognition algorithm using NLP techniques
- Interactive TUI with data visualization
- Support for custom log formats via regex
- Comprehensive benchmark suite

### Changed
- Complete rewrite of the core analyzer for better performance
- Migrated to Rayon for parallel processing
- Enhanced JSON log support with schema inference
- Improved anomaly detection with lower false positive rate

### Fixed
- Memory leak in long-running analysis sessions
- Timezone handling in log timestamp parsing
- Performance bottleneck in pattern extraction

## [1.2.0] - 2024-11-15
### Added
- Basic ML capabilities for anomaly detection
- CSV export functionality
- Time-based filtering options
- Support for compressed log files

### Fixed
- Parser error for malformed log entries
- UI performance issues with large log files

## [1.1.0] - 2024-08-03
### Added
- JSON log format support
- Basic visualization capabilities
- Multi-threading for faster processing
- Command-line filtering options

### Changed
- Improved memory usage for large log files
- Enhanced log level detection

## [1.0.0] - 2024-05-20
### Added
- Initial release with basic log parsing capabilities
- Support for standard log formats
- Simple statistical analysis
- Command-line interface

## Commit History Guidelines

To maintain a professional commit history, we follow these conventions:

### Commit Message Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code (formatting, etc)
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing or correcting existing tests
- **chore**: Changes to the build process or auxiliary tools

### Example Commit Messages
```
feat(parser): add support for custom log formats

- Implemented regex-based pattern matching
- Added configuration options for format customization
- Created documentation and examples

Closes #123
```

```
fix(analyzer): resolve memory leak in time-series calculation

The time series analyzer was not properly freeing allocated memory
when processing very large log files, causing OOM errors on some systems.

Fix includes better memory management and early cleanup of unused data.

Fixes #456