"""
LlamaLogAnalyzer MLX Edition - Anomaly Detection Model

This module implements an advanced anomaly detection model for log analysis
using Apple's MLX framework for accelerated ML operations on Apple Silicon.
"""

import numpy as np
import mlx.core as mx
import mlx.nn as nn
from datetime import datetime
from typing import List, Dict, Any, Tuple, Optional
import re

class LogAnomalyDetector:
    """MLX-accelerated log anomaly detection model."""
    
    def __init__(self, threshold: float = 0.7, window_size: int = 50):
        """
        Initialize the anomaly detector.
        
        Args:
            threshold: Detection threshold (0.0-1.0)
            window_size: Size of the sliding window for time-series analysis
        """
        self.threshold = threshold
        self.window_size = window_size
        self.normal_patterns = {}
        self.model_initialized = False
        self.model = self._build_model()
        
    def _build_model(self) -> nn.Module:
        """Build the MLX neural network model for anomaly detection."""
        class AnomalyModel(nn.Module):
            def __init__(self, input_size: int = 10, hidden_size: int = 64):
                super().__init__()
                self.encoder = nn.Sequential(
                    nn.Linear(input_size, hidden_size),
                    nn.ReLU(),
                    nn.Linear(hidden_size, hidden_size // 2),
                    nn.ReLU(),
                )
                
                self.decoder = nn.Sequential(
                    nn.Linear(hidden_size // 2, hidden_size),
                    nn.ReLU(),
                    nn.Linear(hidden_size, input_size),
                )
                
            def __call__(self, x):
                encoded = self.encoder(x)
                decoded = self.decoder(encoded)
                return decoded
        
        return AnomalyModel()
    
    def _extract_features(self, log_entries: List[Dict[str, Any]]) -> mx.array:
        """
        Extract features from log entries for anomaly detection.
        
        Args:
            log_entries: List of log entry dictionaries
            
        Returns:
            MLX array of features
        """
        features = []
        
        # Count log levels
        level_counts = {"INFO": 0, "DEBUG": 0, "WARNING": 0, "ERROR": 0, "CRITICAL": 0}
        
        # Time features
        timestamps = []
        
        # Message features
        message_lengths = []
        
        for entry in log_entries:
            # Update level counts
            level = entry.get("level", "")
            if level in level_counts:
                level_counts[level] += 1
            
            # Extract timestamp
            if "timestamp" in entry and entry["timestamp"]:
                timestamps.append(entry["timestamp"].timestamp())
            
            # Message length
            message = entry.get("message", "")
            message_lengths.append(len(message))
        
        # Normalize level counts
        total_entries = len(log_entries) if log_entries else 1
        level_features = [count / total_entries for count in level_counts.values()]
        
        # Time interval features
        time_features = [0.0, 0.0]  # Mean, std
        if len(timestamps) > 1:
            diffs = np.diff(timestamps)
            time_features = [np.mean(diffs), np.std(diffs)]
        
        # Message features
        msg_features = [0.0, 0.0, 0.0]  # Mean, std, max
        if message_lengths:
            msg_features = [
                np.mean(message_lengths),
                np.std(message_lengths),
                np.max(message_lengths)
            ]
        
        # Combine all features
        features = level_features + time_features + msg_features
        
        # Convert to MLX array and normalize
        return mx.array(features, dtype=mx.float32).reshape(1, -1)
    
    def train(self, normal_logs: List[Dict[str, Any]]) -> None:
        """
        Train the model on normal (non-anomalous) log data.
        
        Args:
            normal_logs: List of normal log entries
        """
        print(f"Training anomaly detection model on {len(normal_logs)} log entries")
        
        # Extract features from normal logs
        features = self._extract_features(normal_logs)
        
        # Simple autoencoder training
        optimizer = mx.optimizers.Adam(learning_rate=0.001)
        
        def loss_fn(model, x):
            pred = model(x)
            return mx.mean((pred - x) ** 2)
        
        # Train for 100 epochs
        for epoch in range(100):
            loss_val, grad = mx.value_and_grad(loss_fn)(self.model, features)
            optimizer.update(self.model, grad)
            
            if (epoch + 1) % 10 == 0:
                print(f"Epoch {epoch+1}/100, Loss: {loss_val.item():.6f}")
        
        # Store the reconstruction error on normal data
        pred = self.model(features)
        self.normal_error = mx.mean((pred - features) ** 2).item()
        print(f"Baseline reconstruction error: {self.normal_error:.6f}")
        
        self.model_initialized = True
    
    def detect_anomalies(self, log_entries: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """
        Detect anomalies in log entries.
        
        Args:
            log_entries: List of log entry dictionaries
            
        Returns:
            List of anomaly dictionaries with details
        """
        if not self.model_initialized:
            print("Warning: Model not trained. Using heuristic-based detection.")
            return self._heuristic_detection(log_entries)
        
        # Process in windows
        anomalies = []
        for i in range(0, len(log_entries), self.window_size):
            window = log_entries[i:i+self.window_size]
            window_anomalies = self._detect_in_window(window, i)
            anomalies.extend(window_anomalies)
        
        return anomalies
    
    def _detect_in_window(self, window: List[Dict[str, Any]], offset: int) -> List[Dict[str, Any]]:
        """Detect anomalies in a window of log entries."""
        if not window:
            return []
            
        features = self._extract_features(window)
        pred = self.model(features)
        error = mx.mean((pred - features) ** 2).item()
        
        # Compare with normal error
        anomaly_score = error / (self.normal_error + 1e-10)
        
        anomalies = []
        if anomaly_score > self.threshold:
            # Find potential anomalous entries
            error_level_entries = [
                i + offset for i, entry in enumerate(window) 
                if entry.get("level") in ["ERROR", "CRITICAL", "WARNING"]
            ]
            
            anomalies.append({
                "anomaly_type": "window_anomaly",
                "confidence": min(anomaly_score / 2.0, 0.99),
                "description": f"Unusual log pattern detected in time window",
                "timestamp": datetime.now(),
                "related_entries": error_level_entries or [offset],
                "severity": 3 if anomaly_score > 2*self.threshold else 2
            })
            
        return anomalies
    
    def _heuristic_detection(self, log_entries: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """Fallback heuristic-based anomaly detection when model isn't trained."""
        if not log_entries:
            return []
            
        anomalies = []
        
        # Check for unusual frequency of error logs
        error_count = sum(1 for entry in log_entries 
                        if entry.get("level") in ["ERROR", "CRITICAL"])
        error_ratio = error_count / len(log_entries)
        
        if error_ratio > 0.1:  # More than 10% errors
            error_indices = [i for i, entry in enumerate(log_entries) 
                           if entry.get("level") in ["ERROR", "CRITICAL"]]
            
            anomalies.append({
                "anomaly_type": "frequency_anomaly",
                "confidence": min(error_ratio * 5, 0.95),
                "description": f"High frequency of error logs detected ({error_count} errors)",
                "timestamp": datetime.now(),
                "related_entries": error_indices,
                "severity": 4 if error_ratio > 0.2 else 3
            })
            
        # Check for repeating errors
        error_messages = {}
        for i, entry in enumerate(log_entries):
            if entry.get("level") in ["ERROR", "CRITICAL"]:
                msg = entry.get("message", "")
                error_messages[msg] = error_messages.get(msg, []) + [i]
        
        for msg, indices in error_messages.items():
            if len(indices) > 2:  # Repeating errors
                anomalies.append({
                    "anomaly_type": "repeating_error",
                    "confidence": 0.85,
                    "description": f"Repeating error detected: {msg[:50]}...",
                    "timestamp": datetime.now(),
                    "related_entries": indices,
                    "severity": 3
                })
        
        return anomalies


def convert_log_entries(log_entries):
    """Convert Rust log entries to Python format for the anomaly detector."""
    converted = []
    for entry in log_entries:
        converted.append({
            "timestamp": entry.get("timestamp"),
            "level": entry.get("level", ""),
            "message": entry.get("message", ""),
            "source": entry.get("source")
        })
    return converted


def detect_anomalies(log_data, threshold=0.7):
    """
    Detect anomalies in log data using the MLX-accelerated model.
    
    Args:
        log_data: Log entries to analyze
        threshold: Detection threshold
        
    Returns:
        List of detected anomalies
    """
    detector = LogAnomalyDetector(threshold=threshold)
    
    # For simplification, we're using heuristic detection
    # In production, you would train the model first
    return detector._heuristic_detection(convert_log_entries(log_data)) 