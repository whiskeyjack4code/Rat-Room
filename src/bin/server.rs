use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    host: String,
    port: u16,
}

struct Client {
    id: usize,
    username: String,
    stream: TcpStream,
}

fn load_config() -> Config {
    let config_str = fs::read_to_string("config.toml").expect("config.toml not found");
    toml::from_str(&config_str)
        .expect("Failed to parse config.toml")
}

fn broadcast_message(
    clients: &Arc<Mutex<Vec<Client>>>,
    message: &str,
    skip_id: Option<usize>,
) {
    let mut client_list = clients.lock().unwrap();
    client_list.retain_mut(|client|{
        if skip_id == Some(client.id) {
            return true;
        }

        match client.stream.write_all(message.as_bytes()) {
            Ok(_) => true,
            Err(e) => {
                println!(
                    "Removing dead client {} (id={}): {e}",
                    client.username, client.id
                );
                false
            }
        }
    });
    println!("Total number of clients after cleanup: {}", client_list.len());
}

fn remove_client(clients: &Arc<Mutex<Vec<Client>>>, id: usize) {
    let mut client_list = clients.lock().unwrap();
    client_list.retain_mut(|client|{
        client.id != id
    });
    println!("Client {id} removed immediately. Total clients: {}", client_list.len());
}

fn send_to_client(stream: &mut TcpStream, message: &str) -> bool {
    match stream.write_all(message.as_bytes()) {
        Ok(_) => true,
        Err(e) => {
            println!("Failed to send message to client: {e}");
            false
        }
    }
}

fn handle_client(mut stream: TcpStream, id: usize, clients: Arc<Mutex<Vec<Client>>>) {

    let mut username_buffer = [0; 1024];
    let username_bytes = match stream.read(&mut username_buffer) {
        Ok(0) => {
            println!("Client {id} disconnected before sending a username");
            return;
        }
        Ok(n) => n,
        Err(e) => {
            println!("Failed to read username from client: {id}: {e}");
            return;
        }
    };

    let username = String::from_utf8_lossy(&username_buffer[..username_bytes]).trim().to_string();

    {
        let clients_list = clients.lock().unwrap();

        if clients_list.iter().any(|client| client.username == username) {
            let error_message = format!("[system] > Username '{username}'is already taken");
            let _ = send_to_client(&mut stream, &error_message);

            println!("Rejected duplicate username {username}");
            return;
        }
    }
    {
        let mut clients_list = clients.lock().unwrap();
        let cloned_stream = stream.try_clone().unwrap();

            let client: Client = Client {
                id,
                username: username.clone(),
                stream: cloned_stream,
            };

            clients_list.push(client);
            println!("Username {username} joined. Total clients: {}", clients_list.len());
        }

    let join_message = format!("[system] > {username} joined the chat");
    broadcast_message(&clients, &join_message, Some(id));

    loop {
        let mut buffer = [0; 1024];
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => {
                println!("{username} disconnected cleanly");
                remove_client(&clients, id);

                let leave_message = format!("[system] > {username} left the chat");
                broadcast_message(&clients, &leave_message, Some(id));

                return;
            }
            Ok(n) => n,
            Err(e) => {
                println!("{username} read failed: {e}");
                remove_client(&clients, id);

                let leave_message = format!("[system] > {username} left the chat");
                broadcast_message(&clients, &leave_message, Some(id));
                return;
            }
        };

        let message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

        if message == "/who" {
            let user_list = {
                let client_list = clients.lock().unwrap();
                client_list.iter()
                    .map(|client| client.username.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            };

            let who_message = format!("[system] > Online users: {user_list}");
            let _ = send_to_client(&mut stream, &who_message);
            continue;
        }

        let full_message = format!("{}: {}", username, message);

        println!("{full_message}");

        broadcast_message(&clients, &full_message, Some(id));
    }
}

fn main() {
    let config = load_config();
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).expect("Failed to bind listener");

    let counter = Arc::new(AtomicUsize::new(1));
    let clients = Arc::new(Mutex::new(Vec::new()));

    println!("Server listening on {addr}");

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