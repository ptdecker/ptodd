use crate::pool::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

/// Default contants for the server.
const DEFAULT_POOL_SIZE: usize = 4;

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
    pub fn new(addr: impl Into<String>) -> Server {
        let addr = addr.into();
        Server {
            addr: addr.clone(),
            listener: TcpListener::bind(&addr).unwrap(),
            pool: ThreadPool::build(DEFAULT_POOL_SIZE).unwrap(),
        }
    }

    pub fn run(&self) {
        println!("Listening for connections on {}.", &self.addr);
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            self.pool.execute(|| handle_connection(stream));
        }
        println!("Shutting down.");
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!(
        "Request ({:?}): {:#?}",
        stream.peer_addr().unwrap(),
        http_request
    );
    let (status_line, filename) = match http_request[0].as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
