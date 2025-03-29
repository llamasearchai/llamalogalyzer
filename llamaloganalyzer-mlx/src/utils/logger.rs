//! Logger configuration
use log::{LevelFilter, SetLoggerError};
use env_logger::Builder;
use std::io::Write;

pub fn init() -> Result<(), SetLoggerError> {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
    Ok(())
}
