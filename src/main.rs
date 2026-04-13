use std::net::TcpListener;

const SOCKET: &str = "127.0.0.1:8888";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    println!("Server listening on {SOCKET}");

    let (_stream, addr) = listener.accept().unwrap();
    println!("Client connected from {addr}");
}