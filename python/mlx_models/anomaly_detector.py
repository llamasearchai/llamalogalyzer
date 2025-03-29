"""
MLX-based anomaly detector for log analysis
"""
import numpy as np
try:
    import mlx.core as mx
    HAS_MLX = True
except ImportError:
    import numpy as np
    HAS_MLX = False
    print("MLX not available, falling back to NumPy")

class LogAnomalyDetector:
    """Anomaly detector for log entries using MLX for acceleration"""
    
    def __init__(self, threshold=0.7, use_mlx=True):
        self.threshold = threshold
        self.use_mlx = use_mlx and HAS_MLX
        
    def detect_frequency_anomalies(self, log_counts, time_window=3600):
        """Detect anomalies in log frequency"""
        if len(log_counts) < 10:
            return []
            
        # Convert to MLX array if available
        data = mx.array(log_counts) if self.use_mlx else np.array(log_counts)
        
        # Simple Z-score anomaly detection
        mean = mx.mean(data) if self.use_mlx else np.mean(data)
        std = mx.std(data) if self.use_mlx else np.std(data)
        
        if std == 0:
            return []
            
        z_scores = mx.abs((data - mean) / std) if self.use_mlx else np.abs((data - mean) / std)
        
        # Convert back to numpy for processing if using MLX
        if self.use_mlx:
            z_scores = z_scores.tolist()
            
        anomalies = []
        for i, score in enumerate(z_scores):
            if score > self.threshold:
                anomalies.append({
                    "index": i,
                    "score": float(score),
                    "type": "frequency",
                    "severity": min(int(score * 2), 5)  # Scale 1-5
                })
                
        return anomalies
        
    def detect_content_anomalies(self, messages, embeddings=None):
        """Detect anomalies in log content using embeddings"""
        # Simplified implementation for demonstration
        # In a real implementation, this would use MLX for embedding calculation
        return [] 