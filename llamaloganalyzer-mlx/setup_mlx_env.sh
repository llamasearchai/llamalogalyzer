#!/bin/bash
set -euo pipefail

echo "Setting up MLX environment..."
python3 -m pip install --upgrade pip

# Create virtual environment if not exists
if [ ! -d "venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv venv
fi

# Activate virtual environment
source venv/bin/activate

# Install dependencies from requirements.txt
echo "Installing requirements from requirements.txt..."
python3 -m pip install -r requirements.txt

echo "MLX environment setup complete!"
echo "To activate the environment, run: source venv/bin/activate"
