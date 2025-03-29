//! Machine learning integration with Python/MLX
pub mod anomaly;

use std::process::Command;
use std::path::Path;

/// Initialize ML environment
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Check if Python and MLX are available
    let has_python = Command::new("python3")
        .arg("--version")
        .output()
        .is_ok();
        
    if !has_python {
        log::warn!("Python not found. ML features will be limited.");
        return Ok(());
    }
    
    // Check for MLX
    let has_mlx = Command::new("python3")
        .arg("-c")
        .arg("import mlx.core; print('MLX available')")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);
        
    if !has_mlx {
        log::warn!("MLX not found. Using fallback implementations.");
    } else {
        log::info!("MLX framework detected. Using hardware acceleration.");
    }
    
    Ok(())
} 