use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

const SOCKET: &str = "127.0.0.1:8888";

fn handle_client(mut stream: TcpStream, id: usize, clients: Arc<Mutex<Vec<usize>>>) {

    {
        let mut clients_list = clients.lock().unwrap();
        clients_list.push(id);
        println!("Connected clients: {:?}", *clients_list);
    }

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    println!(
        "Client {id} says: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );
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