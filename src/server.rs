//! Basic HTTP Server
//!
//! This module implements a basic HTTP server. This server leverages a thread pool to handle
//! pool to handle incoming connections. It has no third-party crate dependencies.
//!
use crate::{pool::ThreadPool, Error, DEFAULT_POOL_SIZE};
use log::{debug, info};
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

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
    pub fn new(addr: impl Into<String>) -> Result<Server, Error> {
        let addr = addr.into();
        Ok(Server {
            addr: addr.clone(),
            listener: TcpListener::bind(&addr)?,
            pool: ThreadPool::build(DEFAULT_POOL_SIZE)?,
        })
    }

    pub fn run(&self) -> Result<(), Error> {
        info!("Listening for connections on {}", &self.addr);
        for stream in self.listener.incoming() {
            self.pool
                .execute(|| handle_connection(stream.unwrap()).unwrap())?;
        }
        info!("Shutting down");
        Ok(())
    }
}

pub fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let http_request: Vec<_> = BufReader::new(&mut stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    info!("{}", http_request[0]);
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
    stream.write_all(
        format!(
            "{status_line}\r\nContent-Length: {}\r\n\r\n{contents}",
            contents.len()
        )
        .as_bytes(),
    )?;
    Ok(())
}
