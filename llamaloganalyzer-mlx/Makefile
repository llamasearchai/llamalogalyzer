.PHONY: all build clean test doc run run-interactive setup-mlx help

all: build

build:
	@echo "Building LlamaLogAnalyzer MLX Edition..."
	cargo build --release

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf target output __pycache__ python/__pycache__
	find . -name "*.pyc" -delete

test:
	@echo "Running tests..."
	cargo test

doc:
	@echo "Generating documentation..."
	cargo doc --no-deps --open

run:
	@echo "Running LlamaLogAnalyzer MLX Edition..."
	cargo run --release

run-interactive:
	@echo "Running in interactive mode..."
	cargo run --release -- --interactive

setup-mlx:
	@echo "Setting up MLX environment..."
	./setup_mlx_env.sh

help:
	@echo "Makefile for LlamaLogAnalyzer MLX Edition"
	@echo ""
	@echo "Available targets:"
	@echo "  all               Build the project (default)"
	@echo "  build             Build the project"
	@echo "  clean             Clean build artifacts"
	@echo "  test              Run tests"
	@echo "  doc               Generate documentation"
	@echo "  run               Run the application"
	@echo "  run-interactive   Run in interactive mode"
	@echo "  setup-mlx         Setup MLX environment"
	@echo "  help              Show this help message"
