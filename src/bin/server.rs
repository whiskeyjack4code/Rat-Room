use std::net::TcpListener;
use std::io::Read;

const SOCKET: &str = "127.0.0.1:8888";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    println!("Server listening on {SOCKET}");

    loop {
        let (mut stream, addr) = listener.accept().unwrap();
        println!("Client connected from {addr}");

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();

        println!("Received {bytes_read} bytes");
        println!("Message: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

    }

}