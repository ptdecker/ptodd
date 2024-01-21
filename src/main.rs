//! Provides the backend implementation for the ptodd.org website.
use log::info;
use ptodd::{Logger, Server};
use std::error::Error;

const DEFAULT_ADDR: &str = "127.0.0.1:6502";

fn main() -> Result<(), Box<dyn Error>> {
    Logger::init()?;
    info!("Attempting to start server");
    Server::new(DEFAULT_ADDR)?.run()?;
    Ok(())
}
