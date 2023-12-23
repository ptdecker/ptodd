use ptodd::{server::handle_connection, ThreadPool};
use std::net::TcpListener;

fn main() {
    let addr = "127.0.0.1:6502";
    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::build(4).unwrap();
    println!("Listening for connections on {}.", addr);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
    println!("Shutting down.");
}
