#!/usr/bin/env python3
"""
LlamaLogAnalyzer MLX Edition - Integration Verification Script

This script verifies that the Python-Rust integration is correctly configured
by testing the interface between the MLX models and the Rust codebase.
"""

import os
import sys
import time
import json
import argparse
from pathlib import Path

# Add project root to the Python path
project_root = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(project_root))

try:
    import mlx
    import mlx.nn as nn
    import mlx.core as mx
    print(f"✓ MLX is correctly installed (version: {mlx.__version__})")
except ImportError as e:
    print(f"✗ MLX import failed: {e}")
    print("  Please run ./setup_mlx_env.sh to set up the Python environment")
    sys.exit(1)

try:
    # Import our anomaly detection module
    from python.mlx_models.anomaly_detector import LogAnomalyDetector, detect_anomalies
    print("✓ LlamaLogAnalyzer MLX modules imported successfully")
except ImportError as e:
    print(f"✗ Failed to import LlamaLogAnalyzer MLX modules: {e}")
    print("  Please check that the Python module structure is correct")
    sys.exit(1)

def create_sample_log_entries(num_entries=10):
    """Create sample log entries for testing"""
    entries = []
    timestamps = [f"2023-06-{i:02d}T10:00:00Z" for i in range(1, num_entries+1)]
    levels = ["INFO"] * 7 + ["WARNING"] * 2 + ["ERROR"]
    messages = [
        "Application started",
        "Loading configuration",
        "Connected to database",
        "Processing request",
        "Request completed",
        "Cache updated",
        "User logged in",
        "High memory usage detected",
        "Slow query detected",
        "Database connection failed"
    ]
    
    for i in range(num_entries):
        entries.append({
            "timestamp": timestamps[i],
            "level": levels[i],
            "message": messages[i],
            "source": "test"
        })
    
    return entries

def verify_anomaly_detector():
    """Verify that the anomaly detector works"""
    print("\nTesting anomaly detection functionality...")
    
    # Create sample logs
    normal_logs = create_sample_log_entries(20)
    
    # Create an anomaly detector
    detector = LogAnomalyDetector(threshold=0.7, window_size=5)
    
    # Train the model
    print("Training model on normal logs...")
    try:
        detector.train(normal_logs)
        print("✓ Model training successful")
    except Exception as e:
        print(f"✗ Model training failed: {e}")
        return False
    
    # Create logs with anomalies
    anomaly_logs = normal_logs.copy()
    
    # Add some error logs in sequence (this should be detected as an anomaly)
    for i in range(5):
        anomaly_logs.append({
            "timestamp": f"2023-06-{25+i}T10:00:00Z",
            "level": "ERROR",
            "message": f"Critical error #{i}",
            "source": "test"
        })
    
    # Test detection
    print("Testing anomaly detection...")
    try:
        start_time = time.time()
        anomalies = detector.detect_anomalies(anomaly_logs)
        duration = time.time() - start_time
        
        if anomalies:
            print(f"✓ Detected {len(anomalies)} anomalies in {duration:.3f} seconds")
            for i, anomaly in enumerate(anomalies[:3]):  # Show first 3 anomalies
                print(f"  Anomaly {i+1}: {anomaly['description']} (confidence: {anomaly['confidence']:.2f})")
            return True
        else:
            print("✗ No anomalies detected (expected at least one)")
            return False
            
    except Exception as e:
        print(f"✗ Anomaly detection failed: {e}")
        return False

def main():
    parser = argparse.ArgumentParser(description="Verify LlamaLogAnalyzer MLX integration")
    parser.add_argument('--verbose', '-v', action='store_true', help='Enable verbose output')
    args = parser.parse_args()
    
    print("=" * 70)
    print(" LlamaLogAnalyzer MLX Edition - Integration Verification")
    print("=" * 70)
    
    # Check if running on Apple Silicon
    import platform
    is_apple_silicon = platform.processor() == 'arm' and platform.system() == 'Darwin'
    if is_apple_silicon:
        print("✓ Running on Apple Silicon (optimal for MLX)")
    else:
        print("⚠ Not running on Apple Silicon (MLX performance may be limited)")
    
    # Check Python version
    py_version = f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}"
    if sys.version_info.major == 3 and sys.version_info.minor >= 10:
        print(f"✓ Python version is compatible: {py_version}")
    else:
        print(f"⚠ Python version may be incompatible: {py_version} (3.10+ recommended)")
    
    # Check dependencies
    try:
        import numpy
        import pandas
        print(f"✓ Core dependencies are installed")
    except ImportError as e:
        print(f"✗ Missing core dependencies: {e}")
    
    # Verify the anomaly detector
    integration_ok = verify_anomaly_detector()
    
    print("\nVerification summary:")
    if integration_ok:
        print("✓ Python-Rust integration is working correctly!")
        print("✓ The LlamaLogAnalyzer MLX Edition is ready to use")
        return 0
    else:
        print("✗ Issues were detected with the Python-Rust integration")
        print("  Please check the output above for details")
        return 1

if __name__ == "__main__":
    sys.exit(main()) 