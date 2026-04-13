use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

const SOCKET: &str = "127.0.0.1:8888";

fn handle_client(mut stream: TcpStream, id: usize, clients: Arc<Mutex<Vec<TcpStream>>>) {

    {
        let mut clients_list = clients.lock().unwrap();
        let cloned_stream = stream.try_clone().unwrap();
        clients_list.push(cloned_stream);
        println!("Client {id} added. Total clients: {}", clients_list.len());
    }

    loop {
        let mut buffer = [0; 1024];
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {id} disconnected cleanly");
                return;
            }
            Ok(n) => n,
            Err(e) => {
                println!("Client {id} read failed: {e}");
                return;
            }
        };

        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Client {id} says: {}", message);

        {
            let mut clients_list = clients.lock().unwrap();

            clients_list.retain_mut(|client_stream| {
                match client_stream.write_all(message.as_bytes()) {
                    Ok(_) => true,
                    Err(e) => {
                        println!("Removing dead client stream: {e}");
                        false
                    }
                }
            });
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    let counter = Arc::new(AtomicUsize::new(1));
    let clients = Arc::new(Mutex::new(Vec::new()));

    println!("Server listening on {SOCKET}");

    loop {
        let (stream, addr) = listener.accept().unwrap();
        println!("Client connected from {addr}");

        let counter = Arc::clone(&counter);
        let clients = Arc::clone(&clients);

       thread::spawn(move || {
           let id = counter.fetch_add(1, Ordering::SeqCst);
           handle_client(stream, id, clients);
       });

    }
}