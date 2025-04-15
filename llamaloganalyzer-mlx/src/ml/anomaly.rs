//! Anomaly detection implementation with MLX integration
use crate::parsers::LogEntry;
use anyhow::Result;
use chrono::Utc;
use log::{error, info};
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Types of anomalies that can be detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyType {
    FrequencyAnomaly,
    ContentAnomaly,
    TimePatternAnomaly,
    WindowAnomaly,
    RepeatingError,
    Custom(String),
}

impl From<String> for AnomalyType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "frequency_anomaly" => AnomalyType::FrequencyAnomaly,
            "content_anomaly" => AnomalyType::ContentAnomaly,
            "time_pattern_anomaly" => AnomalyType::TimePatternAnomaly,
            "window_anomaly" => AnomalyType::WindowAnomaly,
            "repeating_error" => AnomalyType::RepeatingError,
            _ => AnomalyType::Custom(s),
        }
    }
}

/// Results of anomaly detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyResult {
    pub anomaly_type: AnomalyType,
    pub confidence: f64,
    pub description: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub related_entries: Vec<usize>,
    pub severity: u8,
}

/// Anomaly detector with Python MLX integration
pub struct AnomalyDetector {
    threshold: f64,
    verbose: bool,
}

impl AnomalyDetector {
    /// Create a new anomaly detector
    pub fn new(threshold: f64, verbose: bool) -> Self {
        Self { threshold, verbose }
    }
    
    /// Detect anomalies in log entries using MLX
    pub fn detect_anomalies(&self, entries: &[LogEntry]) -> Result<Vec<AnomalyResult>> {
        if entries.is_empty() {
            return Ok(vec![]);
        }
        
        let start = Instant::now();
        info!("Starting anomaly detection on {} log entries", entries.len());
        
        // Convert Rust log entries to Python objects
        let result = Python::with_gil(|py| -> Result<Vec<AnomalyResult>> {
            // Import Python module
            let mlx_module = PyModule::import(py, "python.mlx_models.anomaly_detector")?;
            
            // Create list of log entries
            let py_entries = self.log_entries_to_py_list(py, entries)?;
            
            // Call Python function
            let kwargs = [("threshold", self.threshold)].into_py_dict(py);
            let py_results = mlx_module.getattr("detect_anomalies")?.call(
                (py_entries,),
                Some(kwargs)
            )?;
            
            // Convert Python results back to Rust
            self.py_results_to_rust(py, py_results)
        });
        
        match result {
            Ok(anomalies) => {
                let elapsed = start.elapsed();
                info!(
                    "Anomaly detection completed in {:.2?}, found {} anomalies",
                    elapsed,
                    anomalies.len()
                );
                Ok(anomalies)
            }
            Err(e) => {
                error!("Error in anomaly detection: {}", e);
                Err(e)
            }
        }
    }
    
    /// Train the anomaly detection model
    pub fn train(&self, normal_entries: &[LogEntry]) -> Result<()> {
        if normal_entries.is_empty() {
            return Ok(());
        }
        
        let start = Instant::now();
        info!("Training anomaly detection model on {} entries", normal_entries.len());
        
        // Convert Rust log entries to Python objects
        let result = Python::with_gil(|py| -> Result<()> {
            // Import Python module
            let mlx_module = PyModule::import(py, "python.mlx_models.anomaly_detector")?;
            
            // Create list of log entries
            let py_entries = self.log_entries_to_py_list(py, normal_entries)?;
            
            // Create detector instance
            let detector_class = mlx_module.getattr("LogAnomalyDetector")?;
            let detector = detector_class.call1((self.threshold,))?;
            
            // Train the model
            detector.call_method1("train", (py_entries,))?;
            
            Ok(())
        });
        
        match result {
            Ok(_) => {
                let elapsed = start.elapsed();
                info!("Model training completed in {:.2?}", elapsed);
                Ok(())
            }
            Err(e) => {
                error!("Error in model training: {}", e);
                Err(e)
            }
        }
    }
    
    /// Convert Rust LogEntry objects to Python dictionary list
    fn log_entries_to_py_list<'py>(&self, py: Python<'py>, entries: &[LogEntry]) -> Result<&'py PyList> {
        let py_list = PyList::empty(py);
        
        for entry in entries {
            let dict = PyDict::new(py);
            
            // Set timestamp if available
            if let Some(ts) = entry.timestamp {
                dict.set_item("timestamp", ts.to_rfc3339())?;
            } else {
                dict.set_item("timestamp", py.None())?;
            }
            
            dict.set_item("level", &entry.level)?;
            dict.set_item("message", &entry.message)?;
            
            if let Some(source) = &entry.source {
                dict.set_item("source", source)?;
            } else {
                dict.set_item("source", py.None())?;
            }
            
            py_list.append(dict)?;
        }
        
        Ok(py_list)
    }
    
    /// Convert Python results back to Rust AnomalyResult objects
    fn py_results_to_rust<'py>(&self, py: Python<'py>, py_results: &'py PyAny) -> Result<Vec<AnomalyResult>> {
        let mut results = Vec::new();
        
        let py_list = py_results.extract::<&PyList>()?;
        for item in py_list.iter() {
            let dict = item.downcast::<PyDict>()?;
            
            let anomaly_type = dict.get_item("anomaly_type")
                .ok_or_else(|| anyhow::anyhow!("Missing anomaly_type"))?
                .extract::<String>()?
                .into();
                
            let confidence = dict.get_item("confidence")
                .ok_or_else(|| anyhow::anyhow!("Missing confidence"))?
                .extract::<f64>()?;
                
            let description = dict.get_item("description")
                .ok_or_else(|| anyhow::anyhow!("Missing description"))?
                .extract::<String>()?;
                
            let related_entries = dict.get_item("related_entries")
                .ok_or_else(|| anyhow::anyhow!("Missing related_entries"))?
                .extract::<Vec<usize>>()?;
                
            let severity = dict.get_item("severity")
                .ok_or_else(|| anyhow::anyhow!("Missing severity"))?
                .extract::<u8>()?;
                
            let anomaly = AnomalyResult {
                anomaly_type,
                confidence,
                description,
                timestamp: Utc::now(),  // Use current time since Python may send a different format
                related_entries,
                severity,
            };
            
            results.push(anomaly);
        }
        
        Ok(results)
    }
}
