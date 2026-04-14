use std::net::TcpStream;
use std::thread;
use std::io::{self, Read, Write};

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8888").unwrap();
    println!("Connected to server");

    let username = loop {
        let mut input = String::new();
        println!("Enter your user-name: ");
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if trimmed.is_empty() {
            println!("Username cannot be empty");
            continue;
        }
        break trimmed.to_string()
    };

    stream.write_all(username.as_bytes()).unwrap();

    println!("Welcome {username}!");

    println!("Type messages and press Enter to send.");
    println!("Type /quit to exit.");

    let mut read_stream = stream.try_clone().unwrap();

    thread::spawn(move ||{
       loop {
           let mut buffer = [0; 1024];

           let bytes_read = match read_stream.read(&mut buffer) {
               Ok(0) => {
                   println!("Server closed connection");
                   return;
               }
               Ok(n) => n,
               Err(e) => {
                   println!("Failed to read from server: {}", e);
                   return;
               }
           };
           let message =  String::from_utf8_lossy(&buffer[..bytes_read]);
           println!(
               "Broadcast received: {}",
               message
           );
       }
    });

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if trimmed == "/quit" {
            println!("Disconnecting..");
            break;
        }

        if trimmed.is_empty(){
            continue;
        }

        stream.write_all(trimmed.as_bytes()).unwrap();
    }

}