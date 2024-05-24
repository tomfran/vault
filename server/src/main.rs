mod protocol;

use log::{debug, error, info, LevelFilter};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const BUFFER_SIZE: usize = 4096;

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    info!("Server started!");

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(&mut stream).await;
        });
    }
}

async fn handle_connection(stream: &mut TcpStream) {
    debug!("Handling a new connection");

    let mut buffer = [0; BUFFER_SIZE];

    loop {
        match stream.read(&mut buffer).await {
            Ok(n) => {
                if n == 0 {
                    debug!("Read 0 bytes from client stream, closing handler");
                    break;
                }

                let res = String::from_utf8_lossy(&buffer[..n]);

                debug!("Red client msg: {}", res.trim());

                match stream.write_all(&buffer[0..n]).await {
                    Ok(_) => {
                        debug!("Sent response");
                    }
                    Err(e) => {
                        error!("Error while responsing to client {}", e);
                    }
                };
            }
            Err(e) => {
                error!("Error while reading client message {}", e);
            }
        }
    }

    debug!("Closing handler");
}
