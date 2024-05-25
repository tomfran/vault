use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() {
    match TcpStream::connect("127.0.0.1:6379") {
        Ok(mut stream) => {
            println!("Connected to the server.");

            loop {
                println!("Enter a command (PING, GET <key>, SET <key> <value>):");

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read from stdin");

                let input = input.trim();
                if input.is_empty() {
                    continue;
                }

                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }

                match parts[0].to_uppercase().as_str() {
                    "PING" => {
                        let msg = create_ping_message();
                        stream
                            .write_all(&msg)
                            .expect("Failed to write PING to server");
                    }
                    "GET" => {
                        if parts.len() < 2 {
                            println!("Usage: GET <key>");
                            continue;
                        }
                        let key = parts[1];
                        let msg = create_get_message(key);
                        stream
                            .write_all(&msg)
                            .expect("Failed to write GET to server");
                    }
                    "SET" => {
                        if parts.len() < 3 {
                            println!("Usage: SET <key> <value>");
                            continue;
                        }
                        let key = parts[1];
                        let value = parts[2];
                        let msg = create_set_message(key, value);
                        stream
                            .write_all(&msg)
                            .expect("Failed to write SET to server");
                    }
                    _ => {
                        println!("Unknown command. Available commands: PING, GET <key>, SET <key> <value>");
                        continue;
                    }
                }

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
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn create_ping_message() -> Vec<u8> {
    vec![0, 1] // version 0, instruction 1 (PING)
}

fn create_get_message(key: &str) -> Vec<u8> {
    let mut message = vec![0, 2]; // version 0, instruction 2 (GET)
    let key_bytes = key.as_bytes();
    let key_len = (key_bytes.len() as u32).to_be_bytes();
    message.extend_from_slice(&key_len);
    message.extend_from_slice(key_bytes);
    message
}

fn create_set_message(key: &str, value: &str) -> Vec<u8> {
    let mut message = vec![0, 3]; // version 0, instruction 3 (SET)
    let key_bytes = key.as_bytes();
    let key_len = (key_bytes.len() as u32).to_be_bytes();
    let value_bytes = value.as_bytes();
    let value_len = (value_bytes.len() as u32).to_be_bytes();
    message.extend_from_slice(&key_len);
    message.extend_from_slice(key_bytes);
    message.extend_from_slice(&value_len);
    message.extend_from_slice(value_bytes);
    message
}
