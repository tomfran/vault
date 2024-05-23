use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    println!("SERVER STARTED!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");

                // Buffer to hold the client message
                let mut buffer = [0; 512];

                // Read the message from the client
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        if size > 0 {
                            let received_message = String::from_utf8_lossy(&buffer[..size]);
                            println!("Received message: {}", received_message);

                            // Optionally, send a response back to the client
                            let response = b"Message received";
                            stream.write_all(response).unwrap();
                            println!("Sent response to client");
                        }
                    }
                    Err(e) => {
                        println!("Failed to read from connection: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
