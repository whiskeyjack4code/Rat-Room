use std::io::Write;
use std::net::TcpStream;
use std::io::Read;

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8888").unwrap();
    println!("Connected to server");

    let message = "Hello from the client";
    stream.write_all(message.as_bytes()).unwrap();

    println!("Message sent");

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    println!(
        "Broadcast received: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );

}