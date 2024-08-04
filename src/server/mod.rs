//! Basic HTTP Server
//!
//! This module implements a basic HTTP server. This server leverages a thread pool to handle
//! pool to handle incoming connections. It has no third-party crate dependencies.

use std::{
    fmt, fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    result,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

pub use error::{Error, Result};
use pool::ThreadPool;
use request::Request;

use super::*;

mod error;
mod pool;
mod request;
mod worker;

/// Tread pool size
const DEFAULT_POOL_SIZE: usize = 4;

// TODO: HTTP/1.1 Support
//  https://www.rfc-editor.org/rfc/rfc9110.txt (HTTP Semantics)
//  https://www.rfc-editor.org/rfc/rfc9111.txt (Caching)
//  https://www.rfc-editor.org/rfc/rfc9112.txt (HTTP/1.1)
//      Older: https://www.rfc-editor.org/rfc/rfc2068.txt
// TODO: URI: https://www.rfc-editor.org/rfc/rfc3986.txt
//  https://www.rfc-editor.org/rfc/rfc6454.txt (origin rules)

/// A server, which listens for incoming connections and handles them.
#[derive(Debug)]
pub struct Server {
    /// The address to bind the server to.
    addr: String,
    /// The listener, which listens for incoming connections.
    listener: TcpListener,
    /// The thread pool, which manages our worker threads.
    pool: ThreadPool,
}

impl Server {
    pub fn new(addr: impl Into<String>) -> Result<Server> {
        let addr = addr.into();
        Ok(Server {
            addr: addr.clone(),
            listener: TcpListener::bind(&addr)?,
            pool: ThreadPool::build(DEFAULT_POOL_SIZE)?,
        })
    }

    pub fn run(&self) -> Result<()> {
        info!("Listening for connections on {}", &self.addr);
        for stream_result in self.listener.incoming() {
            self.pool.execute(|| match stream_result {
                Ok(stream) => {
                    handle_connection(stream).unwrap_or_else(|e| warn!("handle_connection: {}", e))
                }
                Err(e) => {
                    warn!("thread: {}", e);
                }
            })?;
        }
        info!("Shutting down");
        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    info!("handling a connection");
    let http_request: Vec<_> = BufReader::new(&mut stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let request = Request::parse(&http_request)?;
    info!("{}", http_request[0]);
    info!("Method: {}", request.method);
    info!("Target: {}", request.target);
    debug!("Request ({:?}): {:#?}", stream.peer_addr()?, http_request);
    let (status_line, filename) = match http_request[0].as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename)?;
    // TODO: Send date in response header (Cf. RFC-9110 6.6.1)
    stream.write_all(
        format!(
            "{status_line}\r\nContent-Length: {}\r\n\r\n{contents}",
            contents.len()
        )
        .as_bytes(),
    )?;
    Ok(())
}
