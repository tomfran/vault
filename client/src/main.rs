use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    match TcpStream::connect("127.0.0.1:6379") {
        Ok(mut stream) => {
            println!("Successfully connected to server at 127.0.0.1:6379");

            // Sending a message to the server
            let msg = b"PING";
            stream.write_all(msg).expect("Failed to write to server");
            println!("Sent message: {:?}", msg);

            // Receiving a response from the server
            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(size) => {
                    println!(
                        "Received message: {}",
                        String::from_utf8_lossy(&buffer[..size])
                    );
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
