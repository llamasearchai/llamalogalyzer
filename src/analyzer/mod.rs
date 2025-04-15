//! Core log analysis functionality
pub mod statistics;
pub mod patterns;
pub mod correlation;
pub mod aggregation;

pub use statistics::{LogStatistics, compute_statistics}; 