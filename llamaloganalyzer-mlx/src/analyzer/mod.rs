//! Core log analysis functionality
pub mod statistics;
pub mod patterns;
pub mod standard;

pub use statistics::{LogStatistics, compute_statistics};
pub use standard::{StandardAnalyzer, AnalysisResults, TimeMetrics, PatternAnalysis};
