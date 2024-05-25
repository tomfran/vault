mod executor;
mod handler;
mod protocol;
mod storage;

use std::sync::{Arc, Mutex};

use log::{debug, error, info, LevelFilter};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::storage::Storage;

const BUFFER_SIZE: usize = 4096;

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    info!("Server started!");

    let storage = Arc::new(Mutex::new(Storage::default()));

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        let storage_clone = Arc::clone(&storage);

        tokio::spawn(async move {
            handle_connection(&mut stream, storage_clone).await;
        });
    }
}

async fn handle_connection(stream: &mut TcpStream, storage: Arc<Mutex<Storage>>) {
    debug!("Handling a new connection");

    let mut buffer = [0; BUFFER_SIZE];

    loop {
        match stream.read(&mut buffer).await {
            Ok(n) => {
                if n == 0 {
                    debug!("Read 0 bytes from client stream, closing handler");
                    break;
                }

                let handler = handler::Handler::new();

                if let Some(response) = handler.handle(&buffer[..n], storage.clone()) {
                    match stream.write_all(&response).await {
                        Ok(_) => {
                            debug!("Sent response");
                        }
                        Err(e) => {
                            error!("Error while responsing to client {}", e);
                        }
                    };
                }
            }
            Err(e) => {
                error!("Error while reading client message {}", e);
            }
        }
    }

    debug!("Closing handler");
}
