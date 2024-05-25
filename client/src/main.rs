use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    match TcpStream::connect("127.0.0.1:6379") {
        Ok(mut stream) => loop {
            let msg: [u8; 2] = [0, 1];

            stream.write_all(&msg).expect("Failed to write to server");

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(size) => {
                    println!("Received message: {:?}", &buffer[..size]);
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }

            sleep(Duration::from_millis(500));
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
