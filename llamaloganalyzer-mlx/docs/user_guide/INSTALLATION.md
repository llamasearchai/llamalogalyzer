# Installation Guide

This guide will walk you through the process of installing LlamaLogAnalyzer MLX Edition and its dependencies.

## System Requirements

- **Operating System**: 
  - macOS 12 (Monterey) or newer (required for MLX acceleration)
  - Linux (without MLX acceleration)
- **Hardware**:
  - For optimal performance: Apple Silicon Mac (M1, M2, or M3 series)
  - Minimum 8GB RAM (16GB recommended for large log files)
  - 1GB free disk space
- **Software Dependencies**:
  - Rust 1.75 or newer
  - Python 3.10 or newer
  - Cargo package manager
  - pip package manager

## Installation Methods

### Method 1: Automatic Installation (Recommended)

The easiest way to install LlamaLogAnalyzer MLX Edition is to use the provided setup script:

1. Clone the repository:
   ```bash
   git clone https://github.com/llamaloganalyzer/llamaloganalyzer-mlx.git
   cd llamaloganalyzer-mlx
   ```

2. Run the main setup script:
   ```bash
   ./main.sh
   ```

   This script will:
   - Check for required dependencies
   - Set up the Rust environment
   - Create the Python virtual environment
   - Install all required packages
   - Build the project

### Method 2: Manual Installation

If you prefer to install manually or the automatic script doesn't work for your environment:

1. Clone the repository:
   ```bash
   git clone https://github.com/llamaloganalyzer/llamaloganalyzer-mlx.git
   cd llamaloganalyzer-mlx
   ```

2. Build the Rust components:
   ```bash
   cargo build --release
   ```

3. Set up the Python environment:
   ```bash
   python -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   pip install -r requirements.txt
   ```

## Verifying Installation

To verify that the installation was successful:

```bash
./verify_installation.sh
```

This script will check that all components are properly installed and configured.

## Installation Troubleshooting

### Common Issues

#### Rust Installation Issues

If you encounter problems with Rust:

1. Make sure you have the latest Rust version:
   ```bash
   rustup update
   ```

2. Check your toolchain:
   ```bash
   rustc --version
   cargo --version
   ```

#### Python Environment Issues

If you have problems with the Python environment:

1. Make sure you're using Python 3.10 or newer:
   ```bash
   python --version
   ```

2. Try reinstalling the Python dependencies:
   ```bash
   pip install --upgrade pip
   pip install -r requirements.txt
   ```

#### MLX Issues

If MLX doesn't work properly:

1. Make sure you're on an Apple Silicon Mac:
   ```bash
   uname -m  # Should output "arm64"
   ```

2. Try reinstalling MLX:
   ```bash
   pip uninstall mlx
   pip install mlx
   ```

### Getting Help

If you continue to experience issues:

1. Check the [Troubleshooting Guide](TROUBLESHOOTING.md)
2. Search for similar issues in the [GitHub Issues](https://github.com/llamaloganalyzer/llamaloganalyzer-mlx/issues)
3. Ask for help in our [Community Forum](https://community.llamaloganalyzer.org)

## Next Steps

After installation, check out the [Getting Started Guide](GETTING_STARTED.md) to learn how to use LlamaLogAnalyzer MLX Edition. 