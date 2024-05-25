use rand::Rng;
use std::io::{Read, Write};
use std::net::TcpStream;

const SERVER_ADDR: &str = "127.0.0.1:6379";
const KEY_MAX_LEN: usize = 10;
const VALUE_MAX_LEN: usize = 20;

fn main() {
    match TcpStream::connect(SERVER_ADDR) {
        Ok(mut stream) => {
            println!("Connected to the server.");

            loop {
                // Generate a random command
                let command = generate_random_command();

                // Send the command to the server
                stream
                    .write_all(&command)
                    .expect("Failed to write to server");

                let mut buffer = [0; 512];
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        println!("Received message: {:?}", &buffer[..size]);
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }

                // Sleep for a while before sending the next command
                // sleep(Duration::from_millis(500));
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn generate_random_command() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let command_type = rng.gen_range(0..3);

    match command_type {
        0 => create_ping_message(),
        1 => create_get_message(&random_string(KEY_MAX_LEN)),
        2 => create_set_message(&random_string(KEY_MAX_LEN), &random_string(VALUE_MAX_LEN)),
        _ => unreachable!(),
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

fn random_string(max_len: usize) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(1..=max_len);
    (0..len)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}
