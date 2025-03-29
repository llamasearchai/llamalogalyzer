//! # LlamaLogAnalyzer MLX Edition
//!
//! A high-performance log analysis library with ML-powered insights.
pub mod analyzer;
pub mod parsers;
pub mod ml;
pub mod cli;
pub mod visualizers;
pub mod utils;

pub const VERSION: &str = "2.0.0";
pub const NAME: &str = "LlamaLogAnalyzer MLX Edition";

pub fn init() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    utils::logger::init()?;
    Ok(())
}
