# Contributing to LlamaLogAnalyzer MLX Edition

Thank you for your interest in contributing to LlamaLogAnalyzer MLX Edition! This document provides guidelines and instructions for contributing to make the process smooth for everyone.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Community](#community)

## Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md) that all contributors are expected to follow. Please read it before participating.

## Getting Started

### Prerequisites

- Rust 1.75 or newer
- Python 3.10 or newer
- Cargo and pip package managers
- Apple Silicon Mac (for MLX acceleration, though contributions can be made on any platform)

### Setting Up the Development Environment

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/llamaloganalyzer-mlx.git
   cd llamaloganalyzer-mlx
   ```
3. Set up the upstream remote:
   ```bash
   git remote add upstream https://github.com/original-owner/llamaloganalyzer-mlx.git
   ```
4. Set up the Python MLX environment:
   ```bash
   ./setup_mlx_env.sh
   ```
5. Build the project:
   ```bash
   cargo build
   ```

## Development Workflow

1. Create a branch for your work:
   ```bash
   git checkout -b feature/your-feature-name
   ```
   
   Use prefixes like:
   - `feature/` for new features
   - `fix/` for bug fixes
   - `docs/` for documentation changes
   - `refactor/` for code refactoring
   - `test/` for adding or modifying tests

2. Make your changes with clear, descriptive commits:
   ```bash
   git commit -m "feat: add support for XYZ log format"
   ```

3. Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for commit messages.

4. Make sure your branch is up to date with upstream:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

## Pull Request Process

1. Push your changes to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. Open a Pull Request (PR) against the `main` branch of the original repository.

3. Fill out the PR template with:
   - A clear description of the changes
   - Any relevant issue numbers (e.g., "Fixes #123")
   - Screenshots or examples if applicable

4. Your PR will be reviewed by maintainers, who may request changes.

5. Once approved, a maintainer will merge your PR.

## Coding Standards

### Rust

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` to format your code
- Ensure `cargo clippy` passes without warnings
- Document public APIs with proper documentation comments

### Python

- Follow [PEP 8](https://www.python.org/dev/peps/pep-0008/) style guide
- Use type hints where appropriate
- Document functions and classes with docstrings

## Testing Guidelines

- Write tests for new features and bug fixes
- Ensure all tests pass before submitting a PR:
  ```bash
  cargo test
  python -m pytest python/tests/
  ```
- Consider adding benchmarks for performance-critical code

## Documentation

- Update documentation to reflect your changes
- Document complex algorithms or unintuitive code with comments
- Add example usage for new features

## Community

- Join our [Discord server](https://discord.gg/example) for discussion
- Subscribe to our [mailing list](mailto:example@example.com) for updates
- Follow the project on [Twitter](https://twitter.com/example)

---

Thank you for contributing to LlamaLogAnalyzer MLX Edition! Your efforts help make this project better for everyone. 