//! # LlamaLogAnalyzer MLX Edition
//!
//! A high-performance log analysis library with ML-powered insights.
//! Combines Rust's concurrent processing with Python MLX integration.
pub mod analyzer;
pub mod parsers;
pub mod ml;
pub mod cli;
pub mod visualizers;
pub mod utils;

pub const VERSION: &str = "2.0.0";
pub const NAME: &str = "LlamaLogAnalyzer MLX Edition";

/// Initialize the library
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    utils::logger::init()?;
    
    // Initialize MLX/Python ML environment if available
    // This is a soft initialization that won't fail if Python/MLX is unavailable
    let _ = ml::init();
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version() {
        assert_eq!(VERSION, "2.0.0");
    }
} 