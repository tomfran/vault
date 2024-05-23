use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const BUFFER_SIZE: usize = 512;

enum Command {
    Ping,
    Set(String, String),
    Get(String),
}

fn main() {
    println!("SERVER STARTED!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                if let Some(cmd) = read_command(&mut s) {
                    handle_command(&mut s, &cmd);
                }
            }
            Err(_) => println!("Error on stream listen"),
        }
    }
}

fn read_command(stream: &mut TcpStream) -> Option<Command> {
    let mut buffer = [0; BUFFER_SIZE];

    match stream.read(&mut buffer) {
        Ok(n) => {
            if n == 0 {
                return None;
            }

            let input = String::from_utf8_lossy(&buffer[..n]);
            let trimmed_input = input.trim();

            string_to_cmd(trimmed_input)
        }
        Err(_) => {
            println!("Error while parsing command");
            None
        }
    }
}

fn string_to_cmd(s: &str) -> Option<Command> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    match parts.as_slice() {
        ["PING"] => Some(Command::Ping),
        ["SET", key, value] => Some(Command::Set(key.to_string(), value.to_string())),
        ["GET", key] => Some(Command::Get(key.to_string())),
        _ => {
            println!("Command not handled '{}'", s);
            None
        }
    }
}

fn handle_command(stream: &mut TcpStream, cmd: &Command) {
    match cmd {
        Command::Ping => {
            println!("Handling PING");
            if stream.write_all(b"PONG").is_err() {
                println!("Error while responding to PING");
            };
        }
        _ => {
            println!("CMD not handled");
        }
    }
}
