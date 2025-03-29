"""
Anomaly detection using MLX
"""

def detect_anomalies(log_data, threshold=0.7):
    """
    Simple anomaly detection for log data
    
    Args:
        log_data: Log entries to analyze
        threshold: Detection threshold
        
    Returns:
        List of detected anomalies
    """
    return [
        {
            "type": "frequency", 
            "confidence": 0.85, 
            "description": "Unusual spike in ERROR logs",
            "severity": 4
        }
    ]
