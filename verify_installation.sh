#!/bin/bash
set -euo pipefail

# ==============================================================================
# 🦙 LLAMALOGANALYZER MLX EDITION - Installation Verification 🦙
# ==============================================================================

# ANSI color definitions
RESET="\033[0m"
BOLD="\033[1m"
UNDERLINE="\033[4m"
BRIGHT_RED="\033[0;91m"
BRIGHT_GREEN="\033[0;92m"
BRIGHT_YELLOW="\033[0;93m"
BRIGHT_BLUE="\033[0;94m"
BRIGHT_MAGENTA="\033[0;95m"
BRIGHT_CYAN="\033[0;96m"
GRAY="\033[0;90m"

echo -e "${BRIGHT_CYAN}${BOLD}════════════════════════════════════════════════════════════${RESET}"
echo -e "${BRIGHT_CYAN}${BOLD}    🦙 LlamaLogAnalyzer MLX Edition - Installation Verification    ${RESET}"
echo -e "${BRIGHT_CYAN}${BOLD}════════════════════════════════════════════════════════════${RESET}\n"

# Define the project directory
PROJECT_DIR="llamaloganalyzer-mlx"

# Check if project directory exists
if [ ! -d "$PROJECT_DIR" ]; then
    echo -e "${BRIGHT_RED}${BOLD}✗ Project directory not found!${RESET}"
    echo -e "${BRIGHT_YELLOW}Please run the main.sh script first to create the project.${RESET}"
    exit 1
fi

cd "$PROJECT_DIR"

# Verify essential files exist
echo -e "${BRIGHT_BLUE}${BOLD}Checking essential files...${RESET}"
ESSENTIAL_FILES=(
    "Cargo.toml"
    "src/lib.rs"
    "src/bin/main.rs"
    "requirements.txt"
    "setup_mlx_env.sh"
    "Makefile"
    "README.md"
)

ALL_FILES_EXIST=true
for file in "${ESSENTIAL_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo -e "${BRIGHT_GREEN}✓ $file found${RESET}"
    else
        echo -e "${BRIGHT_RED}✗ $file not found${RESET}"
        ALL_FILES_EXIST=false
    fi
done

# Check Python virtual environment
echo -e "\n${BRIGHT_BLUE}${BOLD}Checking Python virtual environment...${RESET}"
if [ -d "venv" ]; then
    echo -e "${BRIGHT_GREEN}✓ Python virtual environment found${RESET}"
    # Check if activate script exists
    if [ -f "venv/bin/activate" ]; then
        echo -e "${BRIGHT_GREEN}✓ venv/bin/activate found${RESET}"
    else
        echo -e "${BRIGHT_RED}✗ venv/bin/activate not found${RESET}"
        echo -e "${BRIGHT_YELLOW}Run ./setup_mlx_env.sh to create the virtual environment${RESET}"
    fi
else
    echo -e "${BRIGHT_RED}✗ Python virtual environment not found${RESET}"
    echo -e "${BRIGHT_YELLOW}Run ./setup_mlx_env.sh to create the virtual environment${RESET}"
fi

# Check Rust project is buildable
echo -e "\n${BRIGHT_BLUE}${BOLD}Checking if Rust project is buildable...${RESET}"
if command -v cargo &>/dev/null; then
    echo -e "${BRIGHT_GREEN}✓ Cargo found${RESET}"
    echo -e "${BRIGHT_YELLOW}Attempting to check compilation (without building)...${RESET}"
    if cargo check --quiet; then
        echo -e "${BRIGHT_GREEN}✓ Rust project is compilable${RESET}"
    else
        echo -e "${BRIGHT_RED}✗ Rust project has compilation errors${RESET}"
    fi
else
    echo -e "${BRIGHT_RED}✗ Cargo not found${RESET}"
    echo -e "${BRIGHT_YELLOW}Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${RESET}"
fi

# Final assessment
echo -e "\n${BRIGHT_CYAN}${BOLD}Installation Verification Summary:${RESET}"
if $ALL_FILES_EXIST && [ -d "venv" ] && [ -f "venv/bin/activate" ]; then
    echo -e "${BRIGHT_GREEN}${BOLD}✓ LlamaLogAnalyzer MLX Edition is properly installed!${RESET}"
    echo -e "${BRIGHT_GREEN}${BOLD}✓ You can build the project with 'cargo build --release' or 'make build'${RESET}"
    echo -e "${BRIGHT_GREEN}${BOLD}✓ Run the application with './target/release/llamaloganalyzer-mlx --interactive'${RESET}"
else
    echo -e "${BRIGHT_YELLOW}${BOLD}⚠ Some components are missing or incomplete.${RESET}"
    echo -e "${BRIGHT_YELLOW}${BOLD}⚠ Please address the issues above before running the application.${RESET}"
fi

echo -e "\n${BRIGHT_CYAN}${BOLD}════════════════════════════════════════════════════════════${RESET}"
echo -e "${BRIGHT_YELLOW}🦙 Powered by Rust, MLX, and llama magic 🦙${RESET}" 