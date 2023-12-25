use ptodd::server::Server;

fn main() {
    Server::new("127.0.0.1:6502").run();
}
