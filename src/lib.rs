//! A simple web server library with no crate dependencies beyond the log crate.
pub(crate) mod error;
pub(crate) mod logger;
pub(crate) mod pool;
pub(crate) mod server;
pub(crate) mod time;
pub(crate) mod uri;

// Re-exports
pub use error::Error;
pub use logger::SimpleLogger as Logger;
pub use server::Server;
pub use time::DateTime;

/// Tread pool size
const DEFAULT_POOL_SIZE: usize = 4;
/// Logging level environment variable name
const LOG_ENV_VAR_NAME: &str = "RUST_LOG";

// TODO: HTTP/1.1 Support
//  https://www.rfc-editor.org/rfc/rfc9110.txt (HTTP Semantics)
//  https://www.rfc-editor.org/rfc/rfc9111.txt (Caching)
//  https://www.rfc-editor.org/rfc/rfc9112.txt (HTTP/1.1)
//      Older: https://www.rfc-editor.org/rfc/rfc2068.txt
// TODO: URI: https://www.rfc-editor.org/rfc/rfc3986.txt
//  https://www.rfc-editor.org/rfc/rfc6454.txt (origin rules)
