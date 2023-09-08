use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::Rng; // Import rand
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tokio::time::Duration;

fn read_commands_from_file(file_path: &str) -> Result<Vec<String>, io::Error> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;

    let commands: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    Ok(commands)
}


// Define a structure to hold client information
struct ClientInfo {
    id: u64,
    unique_name: String,
}

pub async fn run_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server listening on 0.0.0.0:8080");

    // Create a HashMap to store client information with IP addresses as keys
    let clients: Arc<Mutex<HashMap<String, ClientInfo>>> = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((socket, addr)) = listener.accept().await {
        // Get the client's IP address as a string
        let ip = addr.ip().to_string();

        // Check if the IP address is already in the HashMap
        let (unique_name, client_id) = match get_or_generate_info(&ip, &clients) {
            Some((name, id)) => (name, id),
            None => {
                let unique_name = generate_random_name();
                let client_id = generate_client_id(&clients);

                // Store client information in the HashMap
                clients.lock().unwrap().insert(ip.clone(), ClientInfo { id: client_id, unique_name: unique_name.clone() });

                (unique_name, client_id)
            }
        };

        // Print the client ID, unique name, and connection status
        println!("Client {} (Unique Name: {}) connected from {}", client_id, unique_name, ip);

        // Send the unique name to the client
        let mut socket = socket;
        socket.write_all(unique_name.as_bytes()).await?;

        // Spawn a new asynchronous task to handle the client
        tokio::spawn(handle_client(socket, client_id));
    }

    Ok(())
}

fn get_or_generate_info(ip: &str, clients: &Arc<Mutex<HashMap<String, ClientInfo>>>) -> Option<(String, u64)> {
    let clients_map = clients.lock().unwrap();
    if let Some(client_info) = clients_map.get(ip) {
        Some((client_info.unique_name.clone(), client_info.id))
    } else {
        None
    }
}

fn generate_random_name() -> String {
    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();

    let name: String = (0..5)
        .map(|_| {
            let idx = rng.gen_range(0..characters.len());
            characters.chars().nth(idx).unwrap()
        })
        .collect();

    name
}

fn generate_client_id(clients: &Arc<Mutex<HashMap<String, ClientInfo>>>) -> u64 {
    // Generate a unique client ID starting from 1
    let clients_map = clients.lock().unwrap();
    let next_id = clients_map.len() as u64 + 1;
    next_id
}


async fn handle_client(socket: TcpStream, client_id: u64) {
    let mut socket = socket;
    let mut buffer = [0; 1024];

    // Read the unique name from the client
    // let mut unique_name = String::new();
    match socket.read(&mut buffer).await {
        Ok(n) if n > 0 => {
            let unique_name = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
            println!("Received unique name from client {}: {}", client_id, unique_name);
        }
        _ => {
            println!("Error reading unique name from client {}", client_id);
            return;
        }
    }

    // Send each command to the client and receive output
    loop {
        // Read the list of commands from a file into a vector
        // let mut status = "true";
        let commands_file_path = "commands.txt"; // Replace with the actual file path
        let commands = match read_commands_from_file(commands_file_path) {
            Ok(commands) => commands,
            Err(e) => {
                eprintln!("Error reading commands from file: {}", e);
                return;
            }
        };
        for command in &commands {
            // Send the command to the client
            if socket.write_all(command.as_bytes()).await.is_err() {
                println!("Error sending command to client {}", client_id);
                println!("Client Disconnected!!!");
                // clients.lock().unwrap().remove(&socket.peer_addr().unwrap().ip().to_string());
                // status = "false";
                return;
            }
            
            let mut output_buffer = [0; 1024];
            let mut output = String::new();
            loop {
                match socket.read(&mut output_buffer).await {
                    Ok(n) if n > 0 => {
                        output.push_str(&String::from_utf8_lossy(&output_buffer[..n]));
                        // Check if the output contains the "done" message
                        if output.contains("done") {
                            break; // Exit the loop if "done" is received
                        }
                    }
                    Ok(_) => {
                        // The client sent an empty message; continue reading
                    }
                    Err(e) => {
                        println!("Error reading output from client {}: {}", client_id, e);
                        break;
                    }
                }
            }

            println!("Received output from client {}:\n{}", client_id, output);
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}