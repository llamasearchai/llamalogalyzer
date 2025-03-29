#!/bin/bash
set -euo pipefail

# ==============================================================================
# 🦙 LLAMALOGANALYZER MLX EDITION 🦙
# Ultimate Concurrent Log Analyzer with ML-powered Anomaly Detection
#
# Description:
#   LlamaLogAnalyzer MLX Edition is a Rust/Python hybrid log analyzer that leverages
#   Apple's MLX framework to concurrently process structured and unstructured log files.
#   It extracts statistics (e.g. log level counts, average time differences), detects anomalies,
#   and visualizes log data using an interactive, llama-themed CLI/TUI.
#
#   Supported log formats include:
#     - Structured: YYYY-MM-DD HH:MM:SS [LEVEL] Message
#     - JSON logs
#     - Syslog and custom formats (via regex)
#
# Features:
#   • Concurrent log processing using Rayon.
#   • ML-powered anomaly detection with MLX acceleration on Apple Silicon M3 Max.
#   • Interactive Terminal UI using Ratatui/crossterm.
#   • Extensive statistics and pattern recognition.
#   • Data visualization with plotters.
# ==============================================================================

# ANSI color definitions
RESET="\033[0m"
BOLD="\033[1m"
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
MAGENTA="\033[0;35m"
CYAN="\033[0;36m"

project_dir="llamaloganalyzer-mlx"
binary_path="$project_dir/target/release/llamaloganalyzer-mlx"

echo -e "${CYAN}${BOLD}═════════════════════════════════════════════════════════${RESET}"
echo -e "${CYAN}${BOLD}       🦙 LLAMALOGANALYZER MLX EDITION v2.0.0 🦙       ${RESET}"
echo -e "${CYAN}${BOLD}═════════════════════════════════════════════════════════${RESET}"

# Check for first-time setup
if [ ! -d "$project_dir" ]; then
    echo -e "${YELLOW}First-time setup detected. Running setup script...${RESET}"
    ./setup.sh
    echo -e "${GREEN}Setup completed!${RESET}"
fi

# Check if the project is built
if [ ! -f "$binary_path" ]; then
    echo -e "${YELLOW}Building LlamaLogAnalyzer MLX Edition...${RESET}"
    cd "$project_dir" && cargo build --release
    echo -e "${GREEN}Build completed!${RESET}"
fi

# Parse arguments
interactive=false
sample=false
help=false
verify=false
file=""

# Process command line arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --interactive|-i)
            interactive=true
            shift
            ;;
        --sample|-s)
            sample=true
            shift
            ;;
        --file|-f)
            file="$2"
            shift 2
            ;;
        --verify|-v)
            verify=true
            shift
            ;;
        --help|-h)
            help=true
            shift
            ;;
        *)
            echo -e "${RED}Unknown option: $1${RESET}"
            help=true
            shift
            ;;
    esac
done

# Show help
if $help; then
    echo -e "${BLUE}${BOLD}Usage:${RESET}"
    echo -e "  ${YELLOW}./main.sh${RESET} [options]"
    echo
    echo -e "${BLUE}${BOLD}Options:${RESET}"
    echo -e "  ${YELLOW}--interactive, -i${RESET}  Run in interactive TUI mode"
    echo -e "  ${YELLOW}--sample, -s${RESET}       Analyze sample log file"
    echo -e "  ${YELLOW}--file, -f <path>${RESET}  Analyze specified log file"
    echo -e "  ${YELLOW}--verify, -v${RESET}       Verify installation"
    echo -e "  ${YELLOW}--help, -h${RESET}         Show this help message"
    exit 0
fi

# Verify installation if requested
if $verify; then
    echo -e "${BLUE}${BOLD}Verifying installation...${RESET}"
    ./verify_installation.sh
    exit 0
fi

# Run the application
cd "$project_dir"

# Ensure Python environment is set up
if [ ! -d "venv" ] || [ ! -f "venv/bin/activate" ]; then
    echo -e "${YELLOW}Setting up Python MLX environment...${RESET}"
    ./setup_mlx_env.sh
    echo -e "${GREEN}Python environment setup completed!${RESET}"
fi

# Interactive mode
if $interactive; then
    echo -e "${BLUE}${BOLD}Starting LlamaLogAnalyzer MLX Edition in interactive mode...${RESET}"
    ./target/release/llamaloganalyzer-mlx --interactive
    exit 0
fi

# Sample mode
if $sample; then
    echo -e "${BLUE}${BOLD}Analyzing sample log file...${RESET}"
    ./target/release/llamaloganalyzer-mlx --file assets/sample_logs/sample.log
    exit 0
fi

# File mode
if [ -n "$file" ]; then
    # Handle both absolute paths and paths relative to the script's location
    if [[ "$file" == /* ]]; then
        # Absolute path
        if [ -f "$file" ]; then
            echo -e "${BLUE}${BOLD}Analyzing log file: $file${RESET}"
            ./target/release/llamaloganalyzer-mlx --file "$file"
        else
            echo -e "${RED}Error: File not found: $file${RESET}"
            exit 1
        fi
    else
        # Relative path - first try relative to current directory
        if [ -f "$file" ]; then
            absolute_path=$(realpath "$file")
            echo -e "${BLUE}${BOLD}Analyzing log file: $absolute_path${RESET}"
            ./target/release/llamaloganalyzer-mlx --file "$absolute_path"
        else
            # Try relative to the script location
            cd ..
            if [ -f "$file" ]; then
                absolute_path=$(realpath "$file")
                cd "$project_dir"
                echo -e "${BLUE}${BOLD}Analyzing log file: $absolute_path${RESET}"
                ./target/release/llamaloganalyzer-mlx --file "$absolute_path"
            else
                cd "$project_dir"
                echo -e "${RED}Error: File not found: $file${RESET}"
                exit 1
            fi
        fi
    fi
    exit 0
fi

# Default behavior if no options provided
echo -e "${YELLOW}No options specified. Run with --help to see available options.${RESET}"
echo -e "${YELLOW}Starting in interactive mode by default...${RESET}"
./target/release/llamaloganalyzer-mlx --interactive