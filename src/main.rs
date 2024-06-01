//! Provides the backend implementation for the ptodd.org website.
use log::info;
use ptodd::{Logger, Server};
use std::error::Error;

const DEFAULT_ADDR: &str = "localhost:6502";

fn main() -> Result<(), Box<dyn Error>> {
    Logger::init()?;
    info!("Attempting to start server");
    Server::new(DEFAULT_ADDR)?.run()?;
    Ok(())
}
