use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;

const SOCKET: &str = "127.0.0.1:8888";

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    println!(
        "Message: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    println!("Server listening on {SOCKET}");

    loop {
        let (stream, addr) = listener.accept().unwrap();
        println!("Client connected from {addr}");

       thread::spawn(move || {
           handle_client(stream)
       });

    }
}