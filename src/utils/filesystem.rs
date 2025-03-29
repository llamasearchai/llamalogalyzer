//! Filesystem utilities
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};

/// Get all log files in a directory
pub fn get_log_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory: {}", dir.display()))?;
        
    let mut log_files = Vec::new();
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
            if extension == "log" || extension == "json" {
                log_files.push(path);
            }
        }
    }
    
    Ok(log_files)
}

/// Create directory if it doesn't exist
pub fn ensure_dir_exists(dir: &Path) -> Result<()> {
    if !dir.exists() {
        fs::create_dir_all(dir)
            .with_context(|| format!("Failed to create directory: {}", dir.display()))?;
    }
    
    Ok(())
} 