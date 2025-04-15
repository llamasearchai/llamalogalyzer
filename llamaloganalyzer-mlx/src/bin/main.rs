use anyhow::{Result, anyhow};
use llamaloganalyzer_mlx::cli::app::run_cli;
use log::info;

fn main() -> Result<()> {
    // Initialize the log system and configuration
    llamaloganalyzer_mlx::init().map_err(|e| anyhow!("Failed to initialize logger: {}", e))?;
    
    info!("Starting LlamaLogAnalyzer MLX Edition v{}", llamaloganalyzer_mlx::VERSION);
    
    // Process command line arguments and run the CLI
    match run_cli() {
        Ok(_) => {
            info!("LlamaLogAnalyzer completed successfully");
            Ok(())
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}
