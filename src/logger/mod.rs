//! Simple logger implementation
//!
//! This logger implementation borrows some code from [simple_logger](https://crates.io/crates/simple_logger)
//! but is a very bare bones logger implementation that makes assumptions where other full-featured
//! loggers would provide configuration options. It is included here in keeping with the intent of this
//! site to minimize (or zero) usage of any third-party crates.
//!
//! Log entries are output to stderr
//!

use std::{
    env::{var, VarError},
    result,
    str::FromStr,
};

use log::{set_boxed_logger, set_max_level, LevelFilter, Log, Metadata, Record};

use crate::time::DateTime;

/// Logging level environment variable name
const LOG_ENV_VAR_NAME: &str = "RUST_LOG";

pub type Result<T> = result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

/// Simple logger
pub struct SimpleLogger {
    log_level: LevelFilter,
}

impl SimpleLogger {
    /// Initialize the logger
    pub fn init() -> Result<()> {
        let log_level = match var(LOG_ENV_VAR_NAME) {
            Ok(result) => Ok(result),
            Err(VarError::NotPresent) => Ok("trace".to_string()),
            Err(e) => Err(e),
        }?;
        let logger = Self {
            log_level: LevelFilter::from_str(log_level.as_str())?,
        };
        set_max_level(logger.log_level);
        set_boxed_logger(Box::new(logger))?;
        Ok(())
    }
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!(
                "{}: {}: {}: {}",
                DateTime::now(),
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
