use std::io::Write;
use std::net::TcpStream;

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8888").unwrap();
    println!("Connected to server");

    let message = "Hello from the client";
    stream.write_all(message.as_bytes()).unwrap();

    println!("Message sent");
}