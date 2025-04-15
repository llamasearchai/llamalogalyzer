# Documentation Guide

This guide provides guidelines for documenting the LlamaLogAnalyzer MLX Edition project. Comprehensive documentation is essential for maintaining the project, onboarding new contributors, and ensuring users can effectively use the software.

## Table of Contents

- [Documentation Structure](#documentation-structure)
- [Code Documentation](#code-documentation)
- [User Documentation](#user-documentation)
- [Markdown Formatting](#markdown-formatting)
- [Documentation Examples](#documentation-examples)

## Documentation Structure

The project documentation is organized as follows:

- **README.md**: Project overview, installation, basic usage examples
- **CONTRIBUTING.md**: Guidelines for contributing to the project
- **CODE_OF_CONDUCT.md**: Community behavior expectations
- **SECURITY.md**: Security policies and vulnerability reporting
- **CHANGELOG.md**: Version history and changes
- **LICENSE**: MIT license details
- **docs/**: Detailed documentation
  - **user_guide/**: User-focused documentation
  - **dev_guide/**: Developer-focused documentation
  - **api/**: API documentation
  - **examples/**: Usage examples

## Code Documentation

### Rust Documentation

- Use doc comments (`///` for single line, `/**` for multi-line) for public APIs
- Document all public functions, structs, enums, and traits
- Include examples in documentation where appropriate
- Document parameters, return values, and potential errors
- Use Markdown formatting in doc comments

Example:

```rust
/// Parses a log entry from a string.
///
/// # Examples
///
/// ```
/// use llamaloganalyzer_mlx::parsers::standard::parse_line;
///
/// let log_line = "2023-05-15 08:12:34 [INFO] Application started";
/// let log_entry = parse_line(log_line);
/// assert!(log_entry.is_some());
/// assert_eq!(log_entry.unwrap().level, "INFO");
/// ```
///
/// # Parameters
///
/// * `line`: The log line to parse
///
/// # Returns
///
/// An `Option<LogEntry>` containing the parsed log entry, or `None` if the line
/// could not be parsed.
pub fn parse_line(line: &str) -> Option<LogEntry> {
    // implementation
}
```

### Python Documentation

- Use docstrings for all public classes and functions
- Use [Google style](https://sphinxcontrib-napoleon.readthedocs.io/en/latest/example_google.html) or [NumPy style](https://numpydoc.readthedocs.io/en/latest/format.html) docstrings
- Include examples, parameters, returns, and raised exceptions

Example:

```python
def detect_anomalies(log_data, threshold=0.7):
    """Detect anomalies in log data using the MLX model.
    
    This function processes a batch of log entries and identifies potential
    anomalies based on the specified threshold.
    
    Args:
        log_data (List[Dict]): List of log entries as dictionaries
        threshold (float, optional): Detection threshold. Defaults to 0.7.
        
    Returns:
        List[Dict]: List of detected anomalies with metadata
        
    Raises:
        ValueError: If log_data is empty or threshold is outside [0, 1] range
    """
    # implementation
```

## User Documentation

User documentation should include:

1. Installation instructions
2. Configuration options
3. Common usage examples
4. Troubleshooting guide
5. FAQ

Use screenshots or diagrams where appropriate to illustrate concepts.

## Markdown Formatting

- Use ATX-style headers (`#` for top level, `##` for second level, etc.)
- Use bulleted lists (`-` or `*`) for unordered lists
- Use numbered lists (`1.`, `2.`, etc.) for ordered lists
- Use backticks (`` ` ``) for inline code
- Use triple backticks (`` ``` ``) for code blocks, specifying the language
- Use `> ` for blockquotes
- Use `---` for horizontal rules
- Use `[link text](URL)` for links
- Use `**text**` for bold, `*text*` for italic

## Documentation Examples

### API Documentation

Every module should have a top-level documentation that explains its purpose and how it fits into the overall architecture:

```rust
//! # Log Analyzer Module
//!
//! This module provides functionality for analyzing log entries and extracting
//! useful information, such as patterns, statistics, and anomalies.
//!
//! ## Architecture
//!
//! The analyzer module is organized as follows:
//!
//! - `standard`: Standard log analysis algorithms
//! - `ml`: Machine learning-based analysis using Python MLX
//! - `visualizers`: Visualization tools for analysis results
```

### User Guide Example

```markdown
# Using the Interactive TUI

The interactive Terminal User Interface (TUI) provides a real-time dashboard
for monitoring log files and analyzing patterns. To launch the TUI:

```bash
./main.sh --interactive
```

## Navigation

- Use **Tab** to switch between views
- Use **Arrow keys** to navigate within a view
- Press **q** to quit the application

## Views

1. **Dashboard**: Overview of log statistics
2. **Logs**: Detailed view of log entries
3. **Analysis**: ML-powered analysis results
```

Remember that good documentation is a continuous process. Update documentation as the code evolves and encourage all contributors to document their changes. 