//! A simple web server library with no crate dependencies beyond the log crate.
pub(crate) mod error;
pub(crate) mod logger;
pub(crate) mod pool;
pub(crate) mod server;
pub(crate) mod time;

// Re-exports
pub use error::Error;
pub use logger::SimpleLogger as Logger;
pub use server::Server;
pub use time::DateTime;

/// Tread pool size
const DEFAULT_POOL_SIZE: usize = 4;
/// Logging level environment variable name
const LOG_ENV_VAR_NAME: &str = "RUST_LOG";
