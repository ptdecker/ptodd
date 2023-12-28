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

/// Defaults
const DEFAULT_POOL_SIZE: usize = 4;
