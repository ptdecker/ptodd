//! Provides the backend implementation for the ptodd.org website.
use log::{info, set_logger, set_max_level, LevelFilter};
use ptodd::{Logger, Server};
use std::error::Error;

const DEFAULT_ADDR: &str = "127.0.0.1:6502";

fn main() -> Result<(), Box<dyn Error>> {
    set_logger(&Logger).unwrap();
    set_max_level(LevelFilter::Info);
    info!("Attempting to start server");
    Server::new(DEFAULT_ADDR)?.run()?;
    Ok(())
}
