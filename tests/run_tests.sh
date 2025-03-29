#!/bin/bash
set -eo pipefail

echo "Running LlamaLogAnalyzer MLX Edition Tests"
echo "========================================"

# Run Rust unit tests
echo "Running Rust unit tests..."
cargo test -- --nocapture

# Test the CLI with sample log files
echo "Testing CLI with sample log files..."
cargo run -- --file assets/sample_logs/sample.log --output json > test_output.json
if [ -s test_output.json ]; then
    echo "✅ CLI output generated successfully"
else
    echo "❌ CLI output test failed"
    exit 1
fi

# Test Python MLX integration
echo "Testing Python MLX integration..."
source venv/bin/activate
python -c "import sys; sys.path.append('python'); from mlx_models.anomaly_detector import LogAnomalyDetector; detector = LogAnomalyDetector(); print('✅ MLX integration test passed')"

echo "All tests completed successfully!" 