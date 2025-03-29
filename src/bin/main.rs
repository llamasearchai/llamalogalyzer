//! LlamaLogAnalyzer MLX Edition
//! Main entry point
use anyhow::Result;
use clap::Parser;
use llamaloganalyzer_mlx::{self, cli::app::{CliArgs, run_cli}};

fn main() -> Result<()> {
    // Initialize the library
    llamaloganalyzer_mlx::init()?;
    
    // Run the CLI
    run_cli()
} 